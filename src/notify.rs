use crate::fetch;
use chrono::Utc;
use cron::Schedule;
use rocket::tokio;
use std::env;
use std::str::FromStr;
use std::time::Duration;

/// Spawn a task that sends a notification about the next topic at the specified cron schedule.
pub fn spawn_send_task() {
    let cron_schedule = env::var("CRON_SCHEDULE").expect("CRON_SCHEDULE not set");
    tokio::task::spawn(async move {
        for datetime in Schedule::from_str(&cron_schedule)
            .expect("Failed to parse cron schedule")
            .upcoming(Utc)
        {
            let duration = datetime - Utc::now();
            println!("Next notification is scheduled for {}", datetime);
            tokio::time::sleep(Duration::from_secs(duration.num_seconds() as u64)).await;

            match super::fs::pop_topic(super::TOPICS_PATH) {
                Some(topic) => match send(&topic).await {
                    Ok(_) => println!("Sent notification with topic: {}", topic),
                    Err(e) => println!("Error sending notification: {}", e),
                },
                None => println!("No topics left to send"),
            };
        }
    });
}

/// Send a Teams MessageCard to a Teams channel webhook with the specified body.
pub async fn send(body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let card = serde_json::json!({
        "@type": "MessageCard",
        "@context": "http://schema.org/extensions",
        "themeColor": "0076D7",
        "summary": "Today's standup topic",
        "sections": [{
            "activityTitle": body,
            "markdown": true
        }, {
            "images": [{
                "image": fetch::fetch_image(body).await?,
            }]
        }]
    });

    println!("Sending card: {:?}", card);

    let response = reqwest::Client::new()
        .post(env::var("WEBHOOK_URL")?)
        .json(&card)
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);

    Ok(())
}
