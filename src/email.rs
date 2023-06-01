use rocket::tokio;
use std::env;

pub fn spawn_send_task() {
    tokio::task::spawn(async move {
        loop {
            match super::fs::pop_topic("topics.txt") {
                Some(topic) => match send(&topic).await {
                    Ok(_) => println!("Sent email with topic: {}", topic),
                    Err(e) => println!("Error sending email: {}", e),
                },
                None => println!("No topics left to send"),
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 12)).await;
        }
    });
}

/// Send an email with the specified body using SendGrid.
pub async fn send(body: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    reqwest::Client::new()
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
    Ok(())
}
