use rocket::{get, routes, launch};

mod fs;
mod fetch;


#[get("/")]
fn index() -> String {
    fs::read_topics().join("\n")
}


#[get("/pop")]
fn pop() -> String {
    fs::pop_topic().unwrap()
}


#[get("/extend")]
async fn extend() -> String {
    let new_suggestions = fetch::parse_suggestions(fetch::fetch_new_suggestions().await);
    fs::append_topics(&new_suggestions).unwrap();
    "Fetched the following topics, which have been added to the list of upcoming topics: \n\n".to_string() + &new_suggestions.join("\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, pop, extend])
}
