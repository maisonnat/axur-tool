use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::OnceLock;
use std::time::Duration;

/// Global database pool
pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();

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

    Ok(())
}
