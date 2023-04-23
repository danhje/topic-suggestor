use std::fs;

/// Read topics from file, if it exists.
pub fn read_topics() -> Vec<String> {
    fs::read_to_string("topics.txt")
        .unwrap_or("".to_string())
        .lines().map(|s| s.to_string())
        .collect()
}

/// Write topics to file.
pub fn append_topics(topics: &Vec<String>) -> std::io::Result<()> {
    fs::write("topics.txt",  read_topics().join("\n") + topics.join("\n").as_str())
}

/// Pop a topic from the file.
pub fn pop_topic() -> std::io::Result<String> {
    let mut topics = read_topics();
    let topic = topics.remove(0);
    fs::write("topics.txt", topics.join("\n"))?;
    Ok(topic)
}