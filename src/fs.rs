use std::collections::HashSet;
use std::fs;


/// Read topics from file, if it exists.
pub fn read_topics(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or("".to_string())
        .lines().map(|s| s.to_string())
        .collect()
}


/// Write topics to file, removing duplicates and empty strings.
pub fn append_topics(topics: &[String], path: &str) -> std::io::Result<()> {
    let mut concatenated: Vec<String> = read_topics(path);
    concatenated.extend(topics.iter().cloned());

    let concatenated: Vec<String> = concatenated
        .into_iter()
        .collect::<HashSet<_>>().into_iter()  // To remove duplicates
        .filter(|s| s.len() > 0)  // Remove empty strings
        .collect::<Vec<String>>();

    fs::write(path, concatenated.join("\n").as_str())
}


/// Pop a topic from the file.
pub fn pop_topic(path: &str) -> std::io::Result<String> {
    let mut topics = read_topics(path);
    let topic = topics.remove(0);
    fs::write(path, topics.join("\n"))?;
    Ok(topic)
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext<'a> {
        topics_path: &'a str,
    }

    fn setup() -> TestContext<'static> {
        let test_file_path = "tests/data/topics.txt";
        let file_content = "Something that inspired you this week.\n\
        A fun fact about yourself.\n\
        One thing you struggled with this week.\n";
        fs::write(&test_file_path, file_content).expect("Failed to create test file");
        TestContext {
            topics_path: test_file_path,
        }
    }

    #[test]
    fn test_read_topics() {
        let ctx = setup();
        let topics = read_topics(ctx.topics_path);
        assert_eq!(topics, vec![
            "Something that inspired you this week.".to_string(),
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
        ]);
    }

    #[test]
    fn test_append_topics() {
        let ctx = setup();
        let new_topics = vec![
            "What got you out of bed today?".to_string(),
            "A fun fact about yourself.".to_string(),  // Duplicate that should be removed
        ];
        append_topics(&new_topics, ctx.topics_path).expect("Failed to append topics");
        let updated_topics = read_topics(ctx.topics_path).sort();
        let expected_topics = vec![
            "Something that inspired you this week.".to_string(),
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
            "What got you out of bed today?".to_string(),
        ].sort();
        assert_eq!(updated_topics, expected_topics);
    }

    #[test]
    fn test_pop_topic() {
        let ctx = setup();
        let popped_topic = pop_topic(ctx.topics_path).expect("Failed to pop topic");
        assert_eq!(popped_topic, "Something that inspired you this week.".to_string());
        let updated_topics = read_topics(ctx.topics_path).sort();
        let expected_topics = vec![
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
        ].sort();
        assert_eq!(updated_topics, expected_topics);
    }
}
