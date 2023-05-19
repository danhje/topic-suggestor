use std::collections::HashSet;
use std::fs;
use rocket::tokio::task;
use crate::fetch;


const MIN_TOPICS: usize = 10;


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
pub async fn pop_topic(path: &str, top_up: bool) -> std::io::Result<String> {
    let mut topics = read_topics(path);
    let topic = topics.remove(0);
    fs::write(path, topics.join("\n"))?;

    if top_up {
        let path_clone = path.to_owned();
        task::spawn(async move {
            top_up_topics(&path_clone).await
        });
    }

    Ok(topic)
}


/// Ensure there are enough topics stored in the file.
pub async fn top_up_topics(path: &str) -> std::io::Result<()> {
    let topics = read_topics(path);
    if topics.len() < MIN_TOPICS {
        let new_suggestions = fetch::parse_suggestions(fetch::fetch_new_suggestions().await);
        append_topics(&new_suggestions, path).unwrap();
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::tokio;
    use uuid::Uuid;

    struct TestContext {
        topics_path: String,
    }

    impl TestContext {
        fn new() -> TestContext {
            let test_file_path = format!("tests/data/{}.txt", Uuid::new_v4());
            let file_content = "Something that inspired you this week.\n\
                A fun fact about yourself.\n\
                One thing you struggled with this week.\n";
            fs::write(&test_file_path, file_content).expect("Failed to create test file");
            TestContext {
                topics_path: test_file_path,
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            fs::remove_file(&self.topics_path).expect("Failed to remove test file");
        }
    }

    #[test]
    fn test_read_topics() {
        let ctx = TestContext::new();
        let topics = read_topics(&ctx.topics_path);
        assert_eq!(topics, vec![
            "Something that inspired you this week.".to_string(),
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
        ]);
    }

    #[test]
    fn test_append_topics() {
        let ctx = TestContext::new();
        let new_topics = vec![
            "What got you out of bed today?".to_string(),
            "A fun fact about yourself.".to_string(),  // Duplicate that should be removed
        ];
        append_topics(&new_topics, &ctx.topics_path).expect("Failed to append topics");
        let updated_topics = read_topics(&ctx.topics_path).sort();
        let expected_topics = vec![
            "Something that inspired you this week.".to_string(),
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
            "What got you out of bed today?".to_string(),
        ].sort();
        assert_eq!(updated_topics, expected_topics);
    }

    #[tokio::test]
    async fn test_pop_topic() {
        let ctx = TestContext::new();
        let popped_topic = pop_topic(&ctx.topics_path, false).await.expect("Failed to pop topic");
        assert_eq!(popped_topic, "Something that inspired you this week.".to_string());
        let updated_topics = read_topics(&ctx.topics_path).sort();
        let expected_topics = vec![
            "A fun fact about yourself.".to_string(),
            "One thing you struggled with this week.".to_string(),
        ].sort();
        assert_eq!(updated_topics, expected_topics);
    }
}
