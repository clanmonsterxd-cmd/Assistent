use reqwest::blocking::get;
use serde_json::Value;

pub fn handle_weather() {
    let url = "https://api.open-meteo.com/v1/forecast\
        ?latitude=50.83&longitude=12.92\
        &current_weather=true";

    match get(url) {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>() {
                if let Some(temp) = json["current_weather"]["temperature"].as_f64() {
                    println!("Aktuelle Temperatur: {:.1} °C", temp);
                } else {
                    println!("Keine Wetterdaten gefunden.");
                }
            }
        }
        Err(_) => println!("Wetter-API nicht erreichbar."),
    }
}
