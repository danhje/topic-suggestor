use rocket::{get, routes};

mod email;
mod fetch;
mod fs;

const TOPICS_PATH: &str = "topics.txt";

#[get("/")]
fn index() -> String {
    fs::read_topics(TOPICS_PATH).join("\n")
}

#[get("/send")]
async fn send() -> String {
    match fs::pop_topic(TOPICS_PATH) {
        Some(topic) => match email::send(&topic).await {
            Ok(_) => format!("Sent email with topic: {}", topic),
            Err(e) => format!("Error sending email: {}", e),
        },
        None => "No topics left to send".to_owned(),
    }
}

#[get("/extend")]
async fn extend() -> String {
    fs::top_up_topics(TOPICS_PATH).await;
    "Done".to_owned()
}

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    fs::top_up_topics(TOPICS_PATH).await;
    email::spawn_send_task();
    rocket::build()
        .mount("/", routes![index, send, extend])
        .launch()
        .await
        .expect("Failed to launch Rocket instance");
}
