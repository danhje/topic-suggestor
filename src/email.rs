use std::env;

/// Send an email with the specified body using SendGrid.
pub async fn send(body: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            "subject": "Topic for today's standup",
            "from": {"email": env::var("SENDER")?},
            "content": [{"type": "text/plain", "value": &format!("Today's topic: {}", &body)}]
        }))
        .send()
        .await?;

    println!("{:#?}", response);

    Ok(())
}
