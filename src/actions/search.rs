use reqwest::blocking::Client;
use serde_json::Value;
use crate::parser::extract_search_query;

pub fn handle_search(query: &str) {
    // Extrahiere die eigentlichen Suchbegriffe
    let search_term = extract_search_query(query);
    
    if search_term.is_empty() {
        println!("Wonach möchtest du suchen?");
        return;
    }
    
    println!("Suche nach: {}\n", search_term);
    
    let url = format!(
        "https://de.wikipedia.org/api/rest_v1/page/summary/{}",
        search_term.replace(" ", "_")
    );

    // Client mit User-Agent Header
    let client = Client::builder()
        .user_agent("LocalKI-Assistant/1.0 (Rust)")
        .build()
        .unwrap();

    match client.get(&url).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<Value>() {
                    if let Some(title) = json["title"].as_str() {
                        println!("=== {} ===\n", title);
                    }
                    
                    if let Some(text) = json["extract"].as_str() {
                        println!("{}\n", text);
                        
                        // Zeige auch den Link
                        if let Some(url) = json["content_urls"]["desktop"]["page"].as_str() {
                            println!("→ Mehr unter: {}", url);
                        }
                    } else {
                        println!("Keine Informationen zu '{}' gefunden.", search_term);
                    }
                } else {
                    println!("Fehler beim Verarbeiten der Antwort.");
                }
            } else {
                println!("Artikel '{}' nicht gefunden (Status: {}).", search_term, resp.status());
            }
        }
        Err(e) => println!("Recherche fehlgeschlagen: {}", e),
    }
}