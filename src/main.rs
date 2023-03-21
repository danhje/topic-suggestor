#[macro_use] extern crate rocket;

extern crate dotenv;

use dotenv::dotenv;
use std::env;




#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fix?<msg>")]
async fn fix(msg: String) -> String {
    dotenv().ok();
    let prompt = "Tell me a fun fact about technology.";
    let client = reqwest::Client::new();

    //https://stackoverflow.com/questions/47911513/how-do-i-set-the-request-headers-using-reqwest
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", env::var("API_KEY").unwrap()))
        .json(&serde_json::json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "max_tokens": 7,
            "temperature": 0
          }))
        .send()
        .await
        .unwrap()
        .text()  // Use json()
        .await
        .unwrap();

    println!("{:?}", response);
    String::from(msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, fix])
}