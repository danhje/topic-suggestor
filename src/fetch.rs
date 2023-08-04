use serde::Deserialize;
use std::env;

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
struct CompletionResponse {
    choices: Vec<Choice>,
}

pub async fn fetch_new_topics() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", env::var("OPENAI_API_KEY")?),
        )
        .json(&serde_json::json!({
          "model": "text-davinci-003",
          "prompt": PROMPT,
          "max_tokens": MODEL_CONTEXT_LENGTH-PROMPT.len(),
          "temperature": 1.0,
          "presence_penalty": 1.0,  // To avoid repetition, like every suggestion ending with "this week"
        }))
        .send()
        .await?;
    let body = response.text().await?;
    let response: CompletionResponse = serde_json::from_str(&body)?;
    let topics = parse_response(response.choices[0].text.clone());
    Ok(topics)
}

fn parse_response(response_text: String) -> Vec<String> {
    response_text
        .lines()
        .map(|s| s.strip_prefix('-').unwrap().trim().to_string())
        .collect()
}

#[derive(Debug, Deserialize)]
struct Url {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ImageGenerationResponse {
    data: Vec<Url>,
}

/// Download image and return its name.
pub async fn fetch_image(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bearer {}", env::var("OPENAI_API_KEY")?),
        )
        .json(&serde_json::json!({
          "prompt": prompt,
          "n": 1,
          "size": "512x512",
        }))
        .send()
        .await?;
    let body = response.text().await?;
    let response: ImageGenerationResponse = serde_json::from_str(&body)?;
    let url = response.data[0].url.clone();

    let parsed_url_path = reqwest::Url::parse(&url)?;
    let path = std::path::Path::new(parsed_url_path.path());
    let filename = path
        .file_name()
        .ok_or("Failed to extract filename")?
        .to_str()
        .ok_or("Failed to convert to str")?;

    let mut file = std::fs::File::create(format!("/var/img/{filename}"))?;
    let response = client.get(&url).send().await?;
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(filename.to_string())
}
