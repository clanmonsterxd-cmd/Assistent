pub fn normalize(text: &str) -> String {
    let cleaned: String = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    cleaned
        .split_whitespace()
        .map(stem)
        .collect::<Vec<_>>()
        .join(" ")
}

fn stem(word: &str) -> String {
    let mut w = word.to_string();

    // Umlaute vereinheitlichen
    w = w.replace("ä", "a")
         .replace("ö", "o")
         .replace("ü", "u");

    // typische deutsche Endungen
    for suffix in [
        "ern", "em", "er", "en",
        "es", "e", "n", "s",
    ] {
        if w.len() > suffix.len() + 2 && w.ends_with(suffix) {
            w.truncate(w.len() - suffix.len());
            break;
        }
    }

    w
}
