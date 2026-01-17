use crate::intent::Intent;

pub fn intent_boost(intent: &Intent, text: &str) -> f32 {
    let t = text.to_lowercase();

    match intent {
        Intent::Greeting => {
            // Reduziere Greeting bei Fragen oder Befehlen
            if t.contains("?") || t.contains("was ist") || t.contains("wer ist") 
                || t.contains("zeig") || t.contains("öffne") || t.contains("starte") {
                0.3  // Stark reduzieren bei Fragen/Befehlen
            } else if t.contains("hallo") || t.contains("hi") || t.contains("hey") || t.contains("guten") {
                1.5
            } else {
                1.0
            }
        }
        Intent::Search => {
            // Prüfe auf Fragestellung
            let is_question = t.contains("was ist") || t.contains("wer ist") || t.contains("wie") 
                || t.contains("warum") || t.contains("wo") || t.contains("?");
            
            if t.contains("suche")
                || t.contains("finde")
                || t.contains("information")
                || t.contains("recherch")
                || t.contains("erkläre")
                || t.contains("über")
                || t.contains("wiki")
                || is_question {
                2.5  // Erhöht von 2.0 um Fragen stärker zu gewichten
            } else {
                1.0
            }
        }
        Intent::Weather => {
            if t.contains("wetter")
                || t.contains("temperatur")
                || t.contains("warm")
                || t.contains("kalt")
                || t.contains("grad")
                || t.contains("regen")
                || t.contains("schnee") {
                1.3
            } else {
                1.0
            }
        }
        Intent::FileOpen => {
            if t.contains("öffne") || t.contains("starte") || t.contains("öffnen") || t.contains("start") {
                1.4
            } else {
                1.0
            }
        }
        Intent::FileShow => {
            if t.contains("zeig") || t.contains("wo ist") || t.contains("finde datei") || t.contains("mir") {
                1.5  // Erhöht von 1.4
            } else {
                1.0
            }
        }
        Intent::Goodbye => {
            if t.contains("danke") || t.contains("tschüss") || t.contains("bis") {
                1.3
            } else {
                1.0
            }
        }
        Intent::Shutdown => {
            if t.contains("schönen") || t.contains("beenden") || t.contains("ausschalten") {
                1.5
            } else {
                1.0
            }
        }
        _ => 1.0,
    }
}