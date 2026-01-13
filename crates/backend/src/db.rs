use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::OnceLock;
use std::time::Duration;
// Deploy trigger: 2026-01-13T15:11

/// Global database pool
pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();
/// Global initialization error
pub static DB_INIT_ERROR: OnceLock<String> = OnceLock::new();

/// Initialize the database connection pool
pub async fn init_db_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // Run migrations/schema creation
    create_schema(&pool).await?;

    // Set the global pool
    let _ = DB_POOL.set(pool.clone());

    Ok(pool)
}

/// Get a reference to the global database pool
pub fn get_db() -> Option<&'static PgPool> {
    DB_POOL.get()
}

/// Get initialization error if any
pub fn get_init_error() -> Option<&'static String> {
    DB_INIT_ERROR.get()
}

/// Create necessary tables if they don't exist
async fn create_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 1. System Logs Table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS system_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            category TEXT NOT NULL,
            message TEXT NOT NULL,
            level TEXT DEFAULT 'info',
            content TEXT,
            metadata JSONB,
            github_path TEXT,
            github_html_url TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Add columns if they don't exist (migrations for existing table)
    let _ = sqlx::query("ALTER TABLE system_logs ADD COLUMN IF NOT EXISTS github_path TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE system_logs ADD COLUMN IF NOT EXISTS github_html_url TEXT")
        .execute(pool)
        .await;

    // Create index on timestamp for fast time-range queries
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_system_logs_timestamp ON system_logs(timestamp);
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on category for filtering
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_system_logs_category ON system_logs(category);
        "#,
    )
    .execute(pool)
    .await?;

    // 2. Admin Users Table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admin_users (
            id SERIAL PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // 2b. Allowed Users Table (Beta Testers / Access Control)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS allowed_users (
            email TEXT PRIMARY KEY,
            role TEXT DEFAULT 'beta_tester',
            description TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            added_by TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;

    // 2c. Beta Requests Table (Public Sign-up)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS beta_requests (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email TEXT NOT NULL,
            company TEXT NOT NULL,
            status TEXT DEFAULT 'pending',
            requested_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            processed_at TIMESTAMP WITH TIME ZONE,
            processed_by TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on status for admin filtering
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_beta_requests_status ON beta_requests(status);
        "#,
    )
    .execute(pool)
    .await?;

    // Seed initial super admin if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM allowed_users")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    if count.0 == 0 {
        let _ = sqlx::query(
            "INSERT INTO allowed_users (email, role, description, added_by) VALUES ($1, $2, $3, $4)",
        )
        .bind("alejandro.maisonnat@axur.com")
        .bind("admin")
        .bind("Initial Super Admin")
        .bind("system")
        .execute(pool)
        .await;
        tracing::info!("Seeded initial super admin: alejandro.maisonnat@axur.com");
    }

    // 3. Analytics Events Table (for structured events)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS analytics_events (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            event_type TEXT NOT NULL,
            tenant_id TEXT,
            user_id TEXT,
            properties JSONB
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on event_type
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_analytics_events_type ON analytics_events(event_type);
        "#,
    )
    .execute(pool)
    .await?;

    // =====================
    // SLIDE EDITOR TABLES
    // =====================

    // 4. Users Table (extended info linked to Axur tenant)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            axur_tenant_id TEXT NOT NULL UNIQUE,
            email TEXT,
            display_name TEXT,
            is_admin BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // 5. User Templates Table (with direct content storage for auto-save)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS user_templates (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT,
            github_path TEXT,
            content JSONB,
            is_public BOOLEAN DEFAULT false,
            preview_image_url TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Migration: Add content column if missing (for existing deployments)
    let _ = sqlx::query("ALTER TABLE user_templates ADD COLUMN IF NOT EXISTS content JSONB")
        .execute(pool)
        .await;
    // Make github_path optional (was required before)
    if let Err(e) = sqlx::query("ALTER TABLE user_templates ALTER COLUMN github_path DROP NOT NULL")
        .execute(pool)
        .await
    {
        tracing::error!("Failed to alter github_path column: {}", e);
    }

    // Migration: Add preview_image_url if missing
    if let Err(e) =
        sqlx::query("ALTER TABLE user_templates ADD COLUMN IF NOT EXISTS preview_image_url TEXT")
            .execute(pool)
            .await
    {
        tracing::error!("Failed to add preview_image_url column: {}", e);
    }

    // Create index on user_id for fast lookup
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_user_templates_user_id ON user_templates(user_id);
        "#,
    )
    .execute(pool)
    .await?;

    // 6. Marketplace Templates Table (published templates)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS marketplace_templates (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            template_id UUID REFERENCES user_templates(id) ON DELETE CASCADE,
            author_id UUID REFERENCES users(id) ON DELETE CASCADE,
            downloads INTEGER DEFAULT 0,
            rating NUMERIC(2,1) DEFAULT 0,
            rating_count INTEGER DEFAULT 0,
            featured BOOLEAN DEFAULT false,
            approved BOOLEAN DEFAULT false,
            published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on approved for marketplace browsing
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_marketplace_approved ON marketplace_templates(approved);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
