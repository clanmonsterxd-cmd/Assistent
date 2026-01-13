use crate::intent::Intent;
use crate::stopwords::*;
use crate::normalize::normalize;

pub fn extract_parameter(intent: &Intent, input: &str) -> Option<String> {
    let clean = normalize(input);
    let words: Vec<&str> = clean.split_whitespace().collect();

    let stopwords = match intent {
        Intent::Search => stopwords_search(),
        Intent::Weather => stopwords_weather(),
        _ => return None,
    };

    let filtered: Vec<&str> = words
        .into_iter()
        .filter(|w| !stopwords.contains(w))
        .collect();

    if filtered.is_empty() {
        None
    } else {
        Some(filtered.join(" "))
    }
}
