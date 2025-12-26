use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = env::var("AXUR_TOKEN").expect("AXUR_TOKEN must be set");
    let customer_env = env::var("AXUR_CUSTOMER").ok(); // Option<String>

    // Tag to search
    let tag = "ax";

    println!("Probing Threat Hunting API (Universal) for Credentials:");
    println!("Token: {}...", &token[0..10]);
    if let Some(c) = &customer_env {
        println!("Customer: {}", c);
    } else {
        println!("Customer: NONE (Using token default)");
    }

    let client = Client::new();
    let url = "https://api.axur.com/gateway/1.0/api/threat-hunting-api/external-search";

    // Queries to test
    let queries = vec![
        (format!("tag:{}", tag), "credential"),
        (format!("tags:{}", tag), "credential"),
        (format!("tag=\"{}\"", tag), "credential"),
        (format!("tags=\"{}\"", tag), "credential"),
        (format!("q=tag:{}", tag), "credential"),
        ("status=\"NEW\"".to_string(), "credential"),
        ("domain=\"netflix.com\"".to_string(), "credential"),
    ];

    for (q, source) in queries {
        println!("\n--- Testing query: '{}' Source: {} ---", q, source);

        let mut body = serde_json::json!({
            "query": q,
            "source": source
        });

        if let Some(c) = &customer_env {
            body.as_object_mut()
                .unwrap()
                .insert("customer".to_string(), serde_json::json!(c));
        }

        println!("Request Body: {}", body);

        let resp = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        println!("Status: {}", status);
        let text = resp.text().await?;
        println!("Response: {}", text);

        if !status.is_success() {
            continue;
        }

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(id) = json.get("searchId").and_then(|v| v.as_str()) {
                println!("Got searchId: {}. Polling...", id);

                // Poll
                let poll_url = format!("{}/{}", url, id);
                for i in 0..5 {
                    sleep(Duration::from_secs(2)).await;
                    let poll_resp = client
                        .get(&poll_url)
                        .header("Authorization", format!("Bearer {}", token))
                        .send()
                        .await?;

                    let poll_text = poll_resp.text().await?;
                    println!("Poll {}: {}", i, poll_text);

                    if poll_text.contains("\"totalResults\":0")
                        && (poll_text.contains("completed") || poll_text.contains("SUCCESSFUL"))
                    {
                        println!("Search completed with 0 results.");
                        break;
                    }

                    if poll_text.contains("completed")
                        || poll_text.contains("SUCCESSFUL")
                        || poll_text.contains("created")
                    {
                        // Check if we actually got items
                        if poll_text.contains("\"totalResults\":")
                            && !poll_text.contains("\"totalResults\":0")
                        {
                            println!("!!! SUCCESS: Got results! !!!");
                        }
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
