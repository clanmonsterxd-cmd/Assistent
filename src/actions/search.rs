use reqwest::blocking::get;
use serde_json::Value;

pub fn handle_search(query: &str) {
    let url = format!(
        "https://de.wikipedia.org/api/rest_v1/page/summary/{}",
        query.replace(" ", "_")
    );

    match get(&url) {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>() {
                if let Some(text) = json["extract"].as_str() {
                    println!("{}", text);
                } else {
                    println!("Keine Informationen gefunden.");
                }
            }
        }
        Err(_) => println!("Recherche fehlgeschlagen."),
    }
}
