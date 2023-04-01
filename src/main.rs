#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_fn_trait_bound)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fix?<msg>")]
fn fix(msg: String) -> String {
    println!("Hello, {msg}!");

    let prompt = "Tell me a fun fact about technology.";

    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", &api_key)).unwrap());
        headers
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .headers(headers)
        .json(&serde_json::json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "max_tokens": 7,
            "temperature": 0
          }))
        .send()
        .await
        .unwrap()
        .json::<OpenAIResponse>()
        .await
        .unwrap();

    response.choices[0].text.clone()
}

#[tokio::main]
async fn main() {
    rocket::ignite().mount("/", routes![index, fix]).launch();

}
