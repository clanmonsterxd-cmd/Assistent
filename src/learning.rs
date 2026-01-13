use crate::intent::Intent;
use crate::classifier::IntentSample;
use crate::vector::Vocab;
use serde::{Serialize, Deserialize};
use std::fs;

const FILE: &str = "learned_samples.json";

#[derive(Serialize, Deserialize)]
pub struct LearningStore {
    pub samples: Vec<IntentSample>,
}

impl LearningStore {
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }

    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(FILE) {
            if let Ok(store) = serde_json::from_str(&data) {
                println!("Gelernte Daten geladen.");
                return store;
            }
        }
        Self::new()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(FILE, json);
        }
    }

    pub fn add_phrase(
        &mut self,
        vocab: &mut Vocab,
        intent: Intent,
        phrase: &str,
    ) {
        let vec = vocab.sentence_vec(phrase);
        self.samples.push(IntentSample {
            intent,
            vector: vec,
            weight: 2.0,
        });
        self.save();
    }

    pub fn decay(&mut self) {
        for s in &mut self.samples {
            s.weight *= 0.995;
            if s.weight < 0.5 {
                s.weight = 0.5;
            }
        }
    }
}
