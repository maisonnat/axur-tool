use serde::Deserialize;
use serde_json::json;
use std::env;
use std::io::{self, Write};

const API_URL: &str = "https://api.axur.com/gateway/1.0/api";

#[derive(Deserialize, Debug)]
struct AxurAuthResponse {
    correlation: Option<String>,
    token: Option<String>,
    #[serde(rename = "deviceId")]
    device_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load .env manually
    println!("Current Dir: {:?}", env::current_dir());
    match dotenv::from_filename(".env") {
        Ok(path) => println!("Loaded .env from {:?}", path),
        Err(e) => {
            println!("dotenv failed for .env: {}", e);
            // Try ../.env
            match dotenv::from_filename("../.env") {
                Ok(path) => println!("Loaded ../.env from {:?}", path),
                Err(e) => println!("dotenv failed for ../.env too: {}", e),
            }
        }
    }

    let client = reqwest::Client::builder().build()?;

    // AUTH FLOW
    let token_bypass = env::var("AXUR_TOKEN").ok();

    let master_token = if let Some(t) = token_bypass {
        println!("Using provided AXUR_TOKEN, skipping login flow.");
        t
    } else {
        println!("No AXUR_TOKEN found, attempting normal login...");
        perform_login(&client).await?
    };

    println!("Master Token acquired: {}...", &master_token[..10]);

    // ==========================================
    // PROBE THREAT HUNTING
    // ==========================================

    let tenant_id = env::var("AXUR_TENANT_ID").unwrap_or_default();
    if tenant_id.is_empty() {
        println!("AXUR_TENANT_ID not set, fetching tenants to find one...");
        let tenants_resp = client
            .get(format!("{}/customers/customers", API_URL))
            .header("Authorization", format!("Bearer {}", master_token))
            .send()
            .await?;

        println!("Tenants Response Status: {}", tenants_resp.status());
        let body = tenants_resp.text().await?;
        println!("{}", body);
        return Ok(());
    }

    println!("Probing Threat Hunting for Tenant: {}", tenant_id);

    // We will search for "example.com" on "signal-lake"
    let source = "signal-lake";
    let query = format!("domain=\"{}\"", "example.com"); // generic probe

    println!("Starting search on {} with query: {}", source, query);

    let search_req = json!({
        "query": query,
        "source": source,
        "customer": tenant_id
    });

    // POST START
    let start_url = format!("{}/threat-hunting-api/external-search", API_URL);
    let start_resp = client
        .post(&start_url)
        .header("Authorization", format!("Bearer {}", master_token))
        .json(&search_req)
        .send()
        .await?;

    println!("POST Response Status: {}", start_resp.status());
    let start_body = start_resp.text().await?;
    println!("POST Response Body: {}", start_body);

    let start_json: serde_json::Value = serde_json::from_str(&start_body).unwrap_or(json!({}));
    let search_id = start_json["searchId"]
        .as_str()
        .or(start_json["id"].as_str());

    if let Some(id) = search_id {
        println!("Got Search ID: {}", id);

        // POLL GET
        let poll_url = format!(
            "{}/threat-hunting-api/external-search/{}?page=1",
            API_URL, id
        );
        println!("Polling GET: {}", poll_url);

        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let poll_resp = client
            .get(&poll_url)
            .header("Authorization", format!("Bearer {}", master_token))
            .send()
            .await?;

        println!("GET Response Status: {}", poll_resp.status());
        let poll_body = poll_resp.text().await?;
        println!("GET Response Body: {}", poll_body);
    } else {
        println!("Failed to get Search ID from response.");
    }

    Ok(())
}

async fn perform_login(client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
    let email_set = env::var("AXUR_EMAIL").is_ok();
    let pass_set = env::var("AXUR_PASSWORD").is_ok();
    println!("Env Check: EMAIL={}, PASSWORD={}", email_set, pass_set);

    if !email_set || !pass_set {
        return Err("Missing credentials in .env".into());
    }

    let email = env::var("AXUR_EMAIL").unwrap();
    let password = env::var("AXUR_PASSWORD").unwrap();

    println!("Authenticating as {}...", email);

    // STEP 1: LOGIN
    let resp = client
        .post(format!("{}/identity/session", API_URL))
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("Login failed: {}", resp.status());
        let dry = resp.text().await?;
        eprintln!("{}", dry);
        return Err("Login failed".into());
    }

    let auth_data: AxurAuthResponse = resp.json().await?;
    let token = auth_data.token.unwrap();
    let correlation = auth_data.correlation.unwrap();

    println!(
        "Step 1 success. Token: {}..., Correlation: {}",
        &token[..10],
        correlation
    );

    // STEP 2: 2FA
    print!("Enter 2FA Code: ");
    io::stdout().flush()?;

    let mut code_input = String::new();
    io::stdin().read_line(&mut code_input)?;
    let code = code_input.trim();

    let resp = client
        .post(format!("{}/identity/session/tfa", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .header("oxref-token", &correlation)
        .json(&json!({ "code": code }))
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("2FA failed: {}", resp.text().await?);
        return Err("2FA failed".into());
    }

    let auth_data: AxurAuthResponse = resp.json().await?;
    let token = auth_data.token.unwrap();
    let device_id = auth_data.device_id.unwrap();

    println!("Step 2 success. Device ID: {}", device_id);

    // STEP 3: FINALIZE
    let resp = client
        .post(format!("{}/identity/session", API_URL))
        .header("Authorization", format!("Bearer {}", token))
        .header("Device-Id", &device_id)
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("Finalize failed: {}", resp.text().await?);
        return Err("Finalize failed".into());
    }

    let auth_data: AxurAuthResponse = resp.json().await?;
    let master_token = auth_data.token.ok_or("No master token")?;

    Ok(master_token)
}
