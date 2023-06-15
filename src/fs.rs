use crate::fetch;
use rocket::tokio;
use std::collections::HashSet;
use std::fs;

const MIN_TOPICS: usize = 10;

/// Read topics from file, if it exists.
pub fn read_topics(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or("".to_string())
        .lines()
        .map(|s| s.to_string())
        .collect()
}

/// Write topics to file, removing duplicates and empty strings.
pub fn append_topics(topics: &[String], path: &str) -> std::io::Result<()> {
    let mut concatenated: Vec<String> = read_topics(path);
    concatenated.extend(topics.iter().cloned());

    let concatenated: Vec<String> = concatenated
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter() // To remove duplicates
        .filter(|s| !s.is_empty()) // Remove empty strings
        .collect::<Vec<String>>();

    fs::write(path, concatenated.join("\n").as_str())
}

/// Pop a topic from the file.
pub fn pop_topic(path: &str) -> Option<String> {
    let path_clone = path.to_owned();
    tokio::task::spawn(async move { top_up_topics(&path_clone).await });

    let mut topics = read_topics(path);
    if topics.is_empty() {
        None
    } else {
        let topic = topics.remove(0);
        match fs::write(path, topics.join("\n")) {
            Ok(_) => {}
            Err(e) => println!("Error writing topics to file: {}", e),
        };
        Some(topic)
    }
}

/// Ensure there are enough topics stored in the file.
pub async fn top_up_topics(path: &str) {
    if read_topics(path).len() >= MIN_TOPICS {
        return;
    }

    println!("Running low on topics, fetching more");
    match fetch::fetch_new_topics().await {
        Ok(new_topics) => {
            match append_topics(&new_topics, path) {
                Ok(_) => println!("Successfully topped up topics"),
                Err(e) => println!("Error appending new topics to file: {}", e),
            };
        }
        Err(e) => println!("Error fetching new topics: {}", e),
    }
}
