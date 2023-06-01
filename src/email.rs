use chrono::Utc;
use cron::Schedule;
use rocket::tokio;
use std::env;
use std::str::FromStr;
use std::time::Duration;

pub fn spawn_send_task() {
    dotenv::dotenv().ok();
    let cron_schedule = env::var("CRON_SCHEDULE").expect("CRON_SCHEDULE not set");
    tokio::task::spawn(async move {
        for datetime in Schedule::from_str(&cron_schedule)
            .expect("Failed to parse cron schedule")
            .upcoming(Utc)
        {
            let duration = datetime - Utc::now();
            println!(
                "Next email is scheduled for {} ({} seconds from now)",
                datetime,
                duration.num_seconds()
            );
            tokio::time::sleep(Duration::from_secs(duration.num_seconds() as u64)).await;

            match super::fs::pop_topic(super::TOPICS_PATH) {
                Some(topic) => match send(&topic).await {
                    Ok(_) => println!("Sent email with topic: {}", topic),
                    Err(e) => println!("Error sending email: {}", e),
                },
                None => println!("No topics left to send"),
            };
        }
    });
}

/// Send an email with the specified body using SendGrid.
pub async fn send(body: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
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
