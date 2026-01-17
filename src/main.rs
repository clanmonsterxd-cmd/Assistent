// Windows: Kein Konsolen-Fenster NUR im Release-Modus
// Im Debug-Modus sehen wir die Konsole für Fehlersuche
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

mod intent;
mod similarity;
mod normalize;
mod vector;
mod classifier;
mod learning;
mod boost;
mod parser;
mod actions;
mod gui;

use intent::Intent;
use vector::Vocab;
use classifier::{IntentSample, classify};
use learning::LearningStore;
use parser::extract_location;

fn main() {
    // Channels für Kommunikation zwischen GUI und Backend
    let (gui_tx, backend_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (backend_tx, gui_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    
    // KEINE kontinuierliche Spracherkennung mehr!
    // Stattdessen: Push-to-Talk Button in der GUI
    
    // Backend-Thread starten
    let backend_tx_clone = backend_tx.clone();
    thread::spawn(move || {
        run_backend(backend_rx, backend_tx_clone);
    });

    // GUI starten (blockiert den Main-Thread)
    if let Err(e) = gui::create_window(gui_tx, gui_rx, backend_tx) {
        eprintln!("GUI-Fehler: {}", e);
    }
}

fn run_backend(rx: Receiver<String>, tx: Sender<String>) {
    let mut vocab = Vocab::new(32);
    let mut learning = LearningStore::load();
    let mut is_active = false;

    let static_samples = vec![
        IntentSample {
            intent: Intent::Greeting,
            vector: vocab.sentence_vec("hallo hi hey guten tag morgen servus grüß"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("wetter temperatur grad celsius vorhersage klima regnet schneit scheint sonne"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("suche finde informationen recherchiere was ist erkläre über wiki wissen"),
            weight: 1.2,
        },
        IntentSample {
            intent: Intent::FileOpen,
            vector: vocab.sentence_vec("öffne starte start öffnen datei programm anwendung app ausführen"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::FileShow,
            vector: vocab.sentence_vec("zeige zeig anzeigen wo ist finde datei ordner mir"),
            weight: 1.2,  // Erhöht
        },
        IntentSample {
            intent: Intent::Goodbye,
            vector: vocab.sentence_vec("tschüss danke bis später wiedersehen auf wiedersehen"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Shutdown,
            vector: vocab.sentence_vec("schönen tag noch abmelden beenden ausschalten ende"),
            weight: 1.0,
        },
    ];

    let _ = tx.send("KI-Assistent gestartet. Warte auf Begrüßung...".to_string());

    // Event-Loop
    while let Ok(input) = rx.recv() {
        if input.trim().is_empty() {
            continue;
        }

        let input_vec = vocab.sentence_vec(&input);
        let mut all_samples = Vec::new();
        all_samples.extend_from_slice(&static_samples);
        all_samples.extend_from_slice(&learning.samples);

        let intent = classify(&input_vec, &input, &all_samples);
        learning.decay();

        match intent {
            Intent::Greeting => {
                if !is_active {
                    is_active = true;
                    let msg = "Hallo! Wie kann ich dir helfen?";
                    let _ = tx.send("======================================".to_string());
                    let _ = tx.send("   KI-Assistent aktiviert!".to_string());
                    let _ = tx.send("======================================".to_string());
                    let _ = tx.send(msg.to_string());
                } else {
                    let msg = "Hallo!";
                    let _ = tx.send(msg.to_string());
                }
            }
            
            Intent::Weather => {
                if !is_active {
                    let _ = tx.send("[Bitte erst begrüßen]".to_string());
                    continue;
                }
                
                let location = extract_location(&input);
                let weather_info = get_weather_info(location);
                let _ = tx.send(weather_info);
            }
            
            Intent::Search => {
                if !is_active {
                    let _ = tx.send("[Bitte erst begrüßen]".to_string());
                    continue;
                }
                
                let search_result = get_search_result(&input);
                let _ = tx.send(search_result);
            }
            
            Intent::FileOpen => {
                if !is_active {
                    let _ = tx.send("[Bitte erst begrüßen]".to_string());
                    continue;
                }
                
                let result = open_file(&input);
                let _ = tx.send(result);
            }
            
            Intent::FileShow => {
                if !is_active {
                    let _ = tx.send("[Bitte erst begrüßen]".to_string());
                    continue;
                }
                
                let result = show_file(&input);
                let _ = tx.send(result);
            }
            
            Intent::Goodbye => {
                if is_active {
                    let msg = "Bis bald! Ich laufe weiter im Hintergrund.";
                    let _ = tx.send(msg.to_string());
                    is_active = false;
                }
            }
            
            Intent::Shutdown => {
                let msg = "Auf Wiedersehen! Schönen Tag noch!";
                let _ = tx.send(msg.to_string());
                learning.save();
                std::process::exit(0);
            }
            
            Intent::Unknown => {
                if !is_active {
                    continue;
                }
                
                let msg = "Ich bin unsicher. Was meintest du?";
                let _ = tx.send(msg.to_string());
            }
        }
    }
}

fn get_weather_info(location: Option<String>) -> String {
    use reqwest::blocking::get;
    use serde_json::Value;

    let (lat, lon, location_name) = if let Some(loc) = location.as_ref() {
        // Geocoding
        let url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=de&format=json",
            loc.replace(" ", "%20")
        );
        
        if let Ok(resp) = get(&url) {
            if let Ok(json) = resp.json::<Value>() {
                if let Some(results) = json["results"].as_array() {
                    if let Some(first) = results.first() {
                        if let (Some(lat), Some(lon)) = (
                            first["latitude"].as_f64(),
                            first["longitude"].as_f64()
                        ) {
                            let name = first["name"].as_str().unwrap_or(loc);
                            (lat, lon, name.to_string())
                        } else {
                            return format!("Ort '{}' nicht gefunden.", loc);
                        }
                    } else {
                        return format!("Ort '{}' nicht gefunden.", loc);
                    }
                } else {
                    return format!("Ort '{}' nicht gefunden.", loc);
                }
            } else {
                (50.83, 12.92, "Auma".to_string())
            }
        } else {
            (50.83, 12.92, "Auma".to_string())
        }
    } else {
        // Kein Ort angegeben - nutze Auma, Thüringen
        (50.83, 12.92, "Auma".to_string())
    };

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&daily=temperature_2m_max,temperature_2m_min,weathercode&timezone=Europe/Berlin",
        lat, lon
    );

    match get(&url) {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>() {
                let mut result = String::new();
                
                result.push_str(&format!("Wetter für {}\n\n", location_name));
                
                if let Some(current) = json["current_weather"].as_object() {
                    if let (Some(temp), Some(code)) = (
                        current["temperature"].as_f64(),
                        current["weathercode"].as_i64()
                    ) {
                        let weather_desc = weather_code_to_text(code);
                        result.push_str(&format!("Aktuell: {:.1}°C, {}\n", temp, weather_desc));
                    }
                }
                
                if let Some(daily) = json["daily"].as_object() {
                    if let (Some(max), Some(min)) = (
                        daily["temperature_2m_max"].as_array().and_then(|a| a.first()?.as_f64()),
                        daily["temperature_2m_min"].as_array().and_then(|a| a.first()?.as_f64())
                    ) {
                        result.push_str(&format!("Heute: {:.1}°C bis {:.1}°C", min, max));
                    }
                }
                
                result
            } else {
                "Fehler beim Abrufen der Wetterdaten.".to_string()
            }
        }
        Err(_) => "Wetter-API nicht erreichbar.".to_string(),
    }
}

fn weather_code_to_text(code: i64) -> &'static str {
    match code {
        0 => "klar",
        1..=3 => "leicht bewölkt",
        45 | 48 => "neblig",
        51..=57 => "leichter Regen",
        61..=67 => "Regen",
        71..=77 => "Schnee",
        80..=82 => "Regenschauer",
        85..=86 => "Schneeschauer",
        95 => "Gewitter",
        96..=99 => "Gewitter mit Hagel",
        _ => "unbekannt",
    }
}

fn get_search_result(query: &str) -> String {
    use reqwest::blocking::Client;
    use serde_json::Value;
    
    let search_term = parser::extract_search_query(query);
    
    let url = format!(
        "https://de.wikipedia.org/api/rest_v1/page/summary/{}",
        search_term.replace(" ", "_")
    );

    let client = Client::builder()
        .user_agent("LocalKI-Assistant/1.0 (Rust)")
        .build()
        .unwrap();

    match client.get(&url).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<Value>() {
                    let mut result = String::new();
                    
                    if let Some(title) = json["title"].as_str() {
                        result.push_str(&format!("=== {} ===\n\n", title));
                    }
                    
                    if let Some(text) = json["extract"].as_str() {
                        result.push_str(text);
                        
                        if let Some(url) = json["content_urls"]["desktop"]["page"].as_str() {
                            result.push_str(&format!("\n\nMehr unter: {}", url));
                        }
                    } else {
                        result = format!("Keine Informationen zu '{}' gefunden.", search_term);
                    }
                    
                    result
                } else {
                    "Fehler beim Verarbeiten der Antwort.".to_string()
                }
            } else {
                format!("Artikel '{}' nicht gefunden.", search_term)
            }
        }
        Err(_) => "Recherche fehlgeschlagen.".to_string(),
    }
}

fn open_file(query: &str) -> String {
    let file_name = parser::extract_file_path(query);
    
    if file_name.is_empty() {
        return "Welche Datei soll ich öffnen?".to_string();
    }
    
    let result_msg = format!("Versuche '{}' zu öffnen...", file_name);
    
    // Clone für den Thread
    let file_name_clone = file_name.clone();
    
    // Nutze die echte Datei-Funktion
    std::thread::spawn(move || {
        use actions::files;
        files::handle_file_open(&format!("Öffne {}", file_name_clone));
    });
    
    result_msg
}

fn show_file(query: &str) -> String {
    let file_name = parser::extract_file_path(query);
    
    if file_name.is_empty() {
        return "Was soll ich anzeigen?".to_string();
    }
    
    let result_msg = format!("Suche '{}'...", file_name);
    
    // Clone für den Thread
    let file_name_clone = file_name.clone();
    
    // Nutze die echte Datei-Funktion
    std::thread::spawn(move || {
        use actions::files;
        files::handle_file_show(&format!("Zeige {}", file_name_clone));
    });
    
    result_msg
}