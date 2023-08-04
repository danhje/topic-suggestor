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

/// Send an AdaptiveCard to a Teams channel webhook with the specified body and an auto-generated image.
pub async fn send(body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image_name = fetch::fetch_image(body).await?;
    let image_url = format!("https://rocket-hello-world-57nl.onrender.com/img/{image_name}");

    let card = serde_json::json!({
       "type":"message",
       "attachments":[
          {
             "contentType": "application/vnd.microsoft.card.adaptive",
             "contentUrl": null,
             "content": {
                "type": "AdaptiveCard",
                "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
                "version": "1.0",
                "action": "type",
                "body": [
                    {
                        "type": "TextBlock",
                        "text": body,
                        "wrap": true,
                    },
                    {
                        "type": "Image",
                        "url": image_url,
                    }
                ]
            }
          }
       ]
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
