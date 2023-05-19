use std::env;

pub async fn send() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", env::var("SENDGRID_API_KEY")?),
        )
        .json(&serde_json::json!({
            "personalizations": [{
                "to": [{"email": env::var("RECIPIENT")?}],
            }],
            "subject": "Hello from Rust",
            "from": {"email": env::var("SENDER")?},
            "content": [{"type": "text/plain", "value": "Hello, world!"}]
        }))
        .send()
        .await?;

    println!("{:#?}", response);

    Ok(())
}
