use crate::normalize::normalize;

/// Extrahiert Suchbegriffe aus einer Suchanfrage
pub fn extract_search_query(input: &str) -> String {
    let t = input.to_lowercase();
    
    // Entferne typische Befehlswörter direkt aus dem Original-Text
    let stopwords = [
        "suche nach ", "suche ", "finde ", "finde informationen ",
        "informationen über ", "über ", "recherchiere ",
        "was ist ein ", "was ist eine ", "was ist der ", "was ist die ", "was ist das ",
        "was ist ", "wer ist ", "erkläre mir ", "erkläre ",
    ];
    
    let mut cleaned = input.to_string();
    
    for stopword in stopwords {
        if t.starts_with(stopword) {
            cleaned = input[stopword.len()..].to_string();
            break;
        }
    }
    
    // Entferne Fragezeichen und Satzzeichen am Ende
    cleaned = cleaned.trim_end_matches(&['?', '.', '!']).trim().to_string();
    
    if cleaned.is_empty() {
        input.to_string()
    } else {
        cleaned
    }
}

/// Extrahiert einen Ortsnamen aus einer Wetteranfrage
pub fn extract_location(input: &str) -> Option<String> {
    let t = input.to_lowercase();
    
    // Suche nach Präpositionen, die einen Ort anzeigen
    let markers = ["in ", "für ", "von ", "bei ", "um "];
    
    for marker in markers {
        if let Some(pos) = t.find(marker) {
            let after = &input[pos + marker.len()..];
            let location = after.split_whitespace()
                .take_while(|w| !w.contains("?"))
                .collect::<Vec<_>>()
                .join(" ");
            
            if !location.is_empty() {
                // Entferne Satzzeichen am Ende
                let clean = location.trim_end_matches(&['.', '?', '!', ',']);
                return Some(clean.trim().to_string());
            }
        }
    }
    
    None
}

/// Extrahiert einen Datei-/Ordnernamen
pub fn extract_file_path(input: &str) -> String {
    let t = input.to_lowercase();
    
    // Suche nach typischen Mustern - längste zuerst!
    let markers = [
        "zeige mir ",
        "zeig mir ",
        "öffne ",
        "zeige ",
        "zeig ",
        "starte ",
        "start ",
    ];
    
    for marker in markers {
        if let Some(pos) = t.find(marker) {
            let after = &input[pos + marker.len()..];
            return after.trim().to_string();
        }
    }
    
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_search() {
        assert_eq!(extract_search_query("suche nach Rust Programmierung"), "rust programmierung");
        assert_eq!(extract_search_query("finde Informationen über Berlin"), "berlin");
    }

    #[test]
    fn test_extract_location() {
        assert_eq!(extract_location("Wie ist das Wetter in Berlin?"), Some("Berlin".to_string()));
        assert_eq!(extract_location("Wetter für München"), Some("München".to_string()));
    }
}