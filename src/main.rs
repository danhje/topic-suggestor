use color_eyre::eyre::Result;
use rocket::{get, routes};

mod fetch;
mod fs;
mod notify;

const TOPICS_PATH: &str = "topics.txt";

#[get("/")]
fn index() -> String {
    fs::read_topics(TOPICS_PATH).join("\n")
}

#[get("/send")]
async fn send() -> String {
    match fs::pop_topic(TOPICS_PATH) {
        Some(topic) => match notify::send(&topic).await {
            Ok(_) => format!("Sent notification with topic: {}", topic),
            Err(e) => format!("Error sending notification: {}", e),
        },
        None => "No topics left to send".to_owned(),
    }
}

#[get("/extend")]
async fn extend() -> String {
    fs::top_up_topics(TOPICS_PATH).await;
    "Done".to_owned()
}

#[get("/pop")]
async fn pop() -> String {
    fs::pop_topic(TOPICS_PATH).unwrap_or("Failed to get topic".to_owned())
}

#[get("/generate/<prompt>")]
async fn generate(prompt: String) -> String {
    fetch::fetch_image(&prompt)
        .await
        .unwrap_or("Failed to download image".to_owned())
}

#[rocket::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    fs::top_up_topics(TOPICS_PATH).await;
    notify::spawn_send_task();
    rocket::build()
        .mount("/", routes![index, send, extend, pop, generate])
        .mount("/img", rocket::fs::FileServer::from("/var/img"))
        .launch()
        .await
        .expect("Failed to launch Rocket instance");
    Ok(())
}
