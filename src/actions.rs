use crate::intent::Intent;

pub fn handle_intent(intent: Intent, text: &str) {
    match intent {
        Intent::Greeting => {
            println!("Hallo!");
        }
        Intent::Weather => {
            println!("(Wetter-Aktion) Anfrage erkannt: {}", text);
        }
        Intent::Search => {
            println!("(Recherche-Aktion) Anfrage erkannt: {}", text);
        }
        Intent::Unknown => {
            println!("Ich habe nicht verstanden, was du möchtest.");
        }
    }
}