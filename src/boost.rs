use crate::intent::Intent;

pub fn intent_boost(intent: &Intent, text: &str) -> f32 {
    let t = text.to_lowercase();

    match intent {
        Intent::Search => {
            if t.contains("suche")
                || t.contains("finde")
                || t.contains("information")
                || t.contains("recherch") {
                1.5
            } else {
                1.0
            }
        }
        Intent::Weather => {
            if t.contains("wetter")
                || t.contains("temperatur")
                || t.contains("warm")
                || t.contains("kalt") {
                1.3
            } else {
                1.0
            }
        }
        _ => 1.0,
    }
}
