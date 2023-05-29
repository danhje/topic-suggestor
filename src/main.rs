use rocket::{get, launch, routes};

mod email;
mod fetch;
mod fs;

#[get("/")]
fn index() -> String {
    fs::read_topics("topics.txt").join("\n")
}

#[get("/send")]
async fn send() -> String {
    match fs::pop_topic("topics.txt", true).await {
        Ok(topic) => match email::send(&topic).await {
            Ok(_) => format!("Sent email with topic: {}", topic),
            Err(e) => format!("Error sending email: {}", e),
        },
        Err(e) => format!("Error popping topic: {}", e),
    }
}

#[get("/extend")]
async fn extend() -> String {
    match fetch::fetch_new_suggestions().await {
        Ok(suggestions) => {
            let new_suggestions = fetch::parse_suggestions(suggestions);
            fs::append_topics(&new_suggestions, "topics.txt").unwrap();
            "Fetched the following topics, which have been added to the list of upcoming topics: \n\n"
                .to_string()
                + &new_suggestions.join("\n")
        }
        Err(e) => format!("Error fetching new suggestions: {}", e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, send, extend])
}
