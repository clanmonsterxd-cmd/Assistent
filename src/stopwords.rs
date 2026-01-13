pub fn stopwords_search() -> Vec<&'static str> {
    vec![
        "finde", "suche", "such", "informationen", "info",
        "über", "zu", "den", "die", "das", "ein", "eine",
    ]
}

pub fn stopwords_weather() -> Vec<&'static str> {
    vec![
        "wie", "ist", "das", "wetter", "temperatur",
        "in", "heute", "jetzt",
    ]
}
