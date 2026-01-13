use std::io::{self, Write};

mod intent;
mod similarity;
mod normalize;
mod vector;
mod classifier;
mod learning;
mod boost;
mod actions;

use intent::Intent;
use vector::Vocab;
use classifier::{IntentSample, classify};
use learning::LearningStore;
use actions::{weather, search};

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
            vector: vocab.sentence_vec("hallo hi hey guten tag"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Weather,
            vector: vocab.sentence_vec("wetter temperatur warm kalt regen sonnig"),
            weight: 1.0,
        },
        IntentSample {
            intent: Intent::Search,
            vector: vocab.sentence_vec("suche finde informationen recherchiere"),
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

        let intent = classify(&input_vec, &input, &all_samples);

        learning.decay();

        match intent {
            Intent::Greeting => println!("Hallo!"),
            Intent::Weather => weather::handle_weather(),
            Intent::Search => search::handle_search(&input),
            Intent::Unknown => {
                println!("Ich bin unsicher.");
                println!("1 = Greeting | 2 = Weather | 3 = Search | 0 = Nichts");

                let choice = read_line();

                match choice.as_str() {
                    "1" => learning.add_phrase(&mut vocab, Intent::Greeting, &input),
                    "2" => learning.add_phrase(&mut vocab, Intent::Weather, &input),
                    "3" => learning.add_phrase(&mut vocab, Intent::Search, &input),
                    _ => println!("Nichts gelernt."),
                }
            }
        }
    }
}
