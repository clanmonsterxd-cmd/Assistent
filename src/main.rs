use std::io::{self, Write};

mod intent;
mod similarity;
mod vector;
mod classifier;
mod learning;
mod normalize;

use intent::Intent;
use vector::Vocab;
use classifier::{IntentSample, classify};
use learning::LearningStore;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut vocab = Vocab::new(32);
    let mut learning = LearningStore::load();

    let static_samples = vec![
        IntentSample {
            intent: Intent::Greeting,
            vector: vocab.sentence_vec("hallo"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("wetter"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("suche"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Greeting,
            vector: vocab.sentence_vec("hi"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("temperatur"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("finde"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Greeting,
            vector: vocab.sentence_vec("hey"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("kalt"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("informationen"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Greeting,
            vector: vocab.sentence_vec("guten tag"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("sonnig"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("recherchiere"),
            weight: 1.0,
        },
    ];

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let input = read_line();

        let input_vec = vocab.sentence_vec(&input);

        let mut all_samples = Vec::new();
        all_samples.extend_from_slice(&static_samples);
        all_samples.extend_from_slice(&learning.samples);

        let intent = classify(&input_vec, &all_samples);
        learning.decay();

        if intent == Intent::Unknown {
            println!("Ich bin unsicher.");
            println!("1 = Greeting | 2 = Weather | 3 = Search | 0 = Nichts");

            let choice = read_line();

            match choice.as_str() {
                "1" => learning.add_phrase(&mut vocab, Intent::Greeting, &input),
                "2" => learning.add_phrase(&mut vocab, Intent::Weather, &input),
                "3" => learning.add_phrase(&mut vocab, Intent::Search, &input),
                _ => println!("Nichts gelernt."),
            }
        } else {
            match intent {
                Intent::Greeting => println!("Hallo!"),
                Intent::Weather => println!("(Wetter erkannt)"),
                Intent::Search => println!("(Suche erkannt)"),
                _ => {}
            }
        }
    }
}
