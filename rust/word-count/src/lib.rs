use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut count: HashMap<String, u32> = HashMap::new();

    for word in words.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        *count.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    count
}
