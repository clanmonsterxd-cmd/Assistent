use std::io::{self, Write};

mod intent;
mod similarity;
mod normalize;
mod vector;
mod classifier;
mod learning;
mod boost;
mod parser;
mod actions;

use intent::Intent;
use vector::Vocab;
use classifier::{IntentSample, classify};
use learning::LearningStore;
use actions::{weather, search, files};
use parser::extract_location;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut vocab = Vocab::new(32);
    let mut learning = LearningStore::load();
    let mut is_active = false; // Fenster-Status

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
            weight: 1.2,  // Erhöht von 1.0
        },
        IntentSample {
            intent: Intent::FileOpen,
            vector: vocab.sentence_vec("öffne starte start öffnen datei programm anwendung app ausführen"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::FileShow,
            vector: vocab.sentence_vec("zeige zeig anzeigen wo ist finde datei ordner"),
            weight: 1.0,
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

    println!("KI-Assistent gestartet. Warte auf Begrüßung...\n");

    loop {
        // Zeige Prompt nur wenn aktiv
        if is_active {
            print!("> ");
            io::stdout().flush().unwrap();
        }
        
        let input = read_line();
        
        if input.is_empty() {
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
                    println!("╔══════════════════════════════════╗");
                    println!("║   KI-Assistent aktiviert!        ║");
                    println!("╚══════════════════════════════════╝");
                    println!("Hallo! Wie kann ich dir helfen?\n");
                } else {
                    println!("Hallo!\n");
                }
            }
            
            Intent::Weather => {
                if !is_active {
                    println!("[Bitte erst begrüßen]\n");
                    continue;
                }
                
                let location = extract_location(&input);
                weather::handle_weather_at_location(location);
                println!();
            }
            
            Intent::Search => {
                if !is_active {
                    println!("[Bitte erst begrüßen]\n");
                    continue;
                }
                
                search::handle_search(&input);
                println!();
            }
            
            Intent::FileOpen => {
                if !is_active {
                    println!("[Bitte erst begrüßen]\n");
                    continue;
                }
                
                files::handle_file_open(&input);
                println!();
            }
            
            Intent::FileShow => {
                if !is_active {
                    println!("[Bitte erst begrüßen]\n");
                    continue;
                }
                
                files::handle_file_show(&input);
                println!();
            }
            
            Intent::Goodbye => {
                if is_active {
                    println!("Bis bald! (Fenster wird minimiert, ich laufe weiter im Hintergrund)\n");
                    is_active = false;
                    println!("╔══════════════════════════════════╗");
                    println!("║   Warte auf Begrüßung...         ║");
                    println!("╚══════════════════════════════════╝\n");
                }
            }
            
            Intent::Shutdown => {
                println!("Auf Wiedersehen! Schönen Tag noch!");
                println!("KI-Assistent wird beendet.\n");
                learning.save();
                break;
            }
            
            Intent::Unknown => {
                if !is_active {
                    // Im Hintergrundmodus keine Interaktion
                    continue;
                }
                
                println!("Ich bin unsicher.");
                println!("1 = Greeting | 2 = Weather | 3 = Search | 4 = FileOpen | 5 = FileShow | 6 = Goodbye | 7 = Shutdown | 0 = Nichts");

                let choice = read_line();

                match choice.as_str() {
                    "1" => learning.add_phrase(&mut vocab, Intent::Greeting, &input),
                    "2" => learning.add_phrase(&mut vocab, Intent::Weather, &input),
                    "3" => learning.add_phrase(&mut vocab, Intent::Search, &input),
                    "4" => learning.add_phrase(&mut vocab, Intent::FileOpen, &input),
                    "5" => learning.add_phrase(&mut vocab, Intent::FileShow, &input),
                    "6" => learning.add_phrase(&mut vocab, Intent::Goodbye, &input),
                    "7" => learning.add_phrase(&mut vocab, Intent::Shutdown, &input),
                    _ => println!("Nichts gelernt."),
                }
                println!();
            }
        }
    }
}