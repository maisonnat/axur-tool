use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Email of the user to add
    #[arg(short, long)]
    email: String,

    /// Role to assign (admin, beta_tester)
    #[arg(short, long, default_value = "beta_tester")]
    role: String,

    /// Optional description
    #[arg(short, long)]
    description: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load .env
    dotenv::dotenv().ok();

    let args = Args::parse();
    let email = args.email.to_lowercase();
    let role = args.role.to_lowercase();

    if role != "admin" && role != "beta_tester" {
        eprintln!("Error: role must be 'admin' or 'beta_tester'");
        process::exit(1);
    }

    println!("Initializing Firebase...");
    // Initialize Firebase
    axur_backend::firebase::init_global().await;

    let firestore = match axur_backend::firebase::get_firestore() {
        Some(f) => f,
        None => {
            eprintln!(
                "Error: Failed to initialize Firestore. Check FIREBASE_PROJECT_ID and credentials."
            );
            process::exit(1);
        }
    };

    let doc_id = email.replace("@", "_at_").replace(".", "_dot_");

    let user = serde_json::json!({
        "email": email,
        "role": role,
        "description": args.description.unwrap_or_else(|| "Added via CLI seeder".to_string()),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "added_by": "cli_admin"
    });

    println!("Adding user {} as {}...", email, role);

    match firestore.set_doc("allowed_users", &doc_id, &user).await {
        Ok(_) => println!("Successfully added user: {}", email),
        Err(e) => {
            eprintln!("Failed to add user: {}", e);
            process::exit(1);
        }
    }
}
