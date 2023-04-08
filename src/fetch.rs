use std::env;
use dotenv::dotenv;
use serde::Deserialize;


const MODEL_CONTEXT_LENGTH: usize = 2048;
const PROMPT: &str = r#"
The Data Science department at Statnett SF has daily "standups", but with a twist. They start the
standup by determining todays standup topic / question. An example of a topic might be "Something
you learned this week". With this example topic, each employee would give an example of something
they learned that week, either at work, or outside work. The topics will often, though not always,
focus on the current week. They are sometimes designed to generate some laughter, other times they
are playfully challenging, and sometimes more serious. Occasionally they are about technical topics
that relate to data science or electrical power transmission. Here are the 20 most recent topics:
- Your number one goal for this week.
- Your favourite Christmas gift this year.
- Your favourite YouTube channel.
- Your biggest blunder this week.
- Praising someone who helped you this week.
- A new perspective you gained this week.
- One TLA you learned this week.
"#;


#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    choices: Vec<Choice>
}


pub async fn fetch_new_suggestions() -> String {
    dotenv().ok();
    let client = reqwest::Client::new();
    let response: Response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", env::var("API_KEY").unwrap()))
        .json(&serde_json::json!({
            "model": "text-davinci-003",
            "prompt": PROMPT,
            "max_tokens": MODEL_CONTEXT_LENGTH-PROMPT.len(),
            "temperature": 1.0,
            "presence_penalty": 1.0,  // To avoid repetition, like every suggestion ending with "this week"
          }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    response.choices[0].text.clone()
}

pub fn parse_suggestions(suggestions: String) -> Vec<String> {
    suggestions
        .lines()
        .map(|s| s.strip_prefix('-').unwrap().trim().to_string())
        .collect()
}