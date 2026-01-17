use reqwest::blocking::get;
use serde_json::Value;

// Geocoding API um Ortsnamen in Koordinaten umzuwandeln
fn get_coordinates(location: &str) -> Option<(f64, f64)> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=de&format=json",
        location.replace(" ", "%20")
    );
    
    match get(&url) {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>() {
                if let Some(results) = json["results"].as_array() {
                    if let Some(first) = results.first() {
                        let lat = first["latitude"].as_f64()?;
                        let lon = first["longitude"].as_f64()?;
                        return Some((lat, lon));
                    }
                }
            }
        }
        Err(_) => {}
    }
    None
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

pub fn handle_weather_at_location(location: Option<String>) {
    // Standard-Koordinaten (Auma, Thüringen)
    let (lat, lon) = if let Some(loc) = location {
        println!("Suche Wetter für: {}", loc);
        match get_coordinates(&loc) {
            Some(coords) => coords,
            None => {
                println!("Ort nicht gefunden. Verwende Standardort.");
                (50.83, 12.92)
            }
        }
    } else {
        (50.83, 12.92)
    };

    let url = format!(
        "https://api.open-meteo.com/v1/forecast\
        ?latitude={}&longitude={}\
        &current_weather=true\
        &daily=temperature_2m_max,temperature_2m_min,weathercode\
        &timezone=Europe/Berlin",
        lat, lon
    );

    match get(&url) {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>() {
                // Aktuelles Wetter
                if let Some(current) = json["current_weather"].as_object() {
                    if let (Some(temp), Some(code)) = (
                        current["temperature"].as_f64(),
                        current["weathercode"].as_i64()
                    ) {
                        println!("Aktuell: {:.1} °C, {}", temp, weather_code_to_text(code));
                    }
                }
                
                // Vorhersage für heute
                if let Some(daily) = json["daily"].as_object() {
                    if let (Some(max), Some(min)) = (
                        daily["temperature_2m_max"].as_array().and_then(|a| a.first()?.as_f64()),
                        daily["temperature_2m_min"].as_array().and_then(|a| a.first()?.as_f64())
                    ) {
                        println!("Heute: {:.1} °C bis {:.1} °C", min, max);
                    }
                }
            }
        }
        Err(_) => println!("Wetter-API nicht erreichbar."),
    }
}

// Kompatibilität mit alter Funktion
pub fn handle_weather() {
    handle_weather_at_location(None);
}