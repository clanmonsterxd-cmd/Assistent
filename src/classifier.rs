use crate::intent::Intent;
use crate::similarity::cosine_similarity;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IntentSample {
    pub intent: Intent,
    pub vector: Vec<f32>,
    pub weight: f32,
}


pub fn classify(input_vec: &[f32], samples: &[IntentSample]) -> Intent {
    let mut best_score = -1.0;
    let mut best_intent = Intent::Unknown;

    for sample in samples {
        let similarity = cosine_similarity(input_vec, &sample.vector);
        let score = similarity * sample.weight;

        //println!("Ähnlichkeit zu {:?}: sim={:.3}, weight={:.2}, score={:.3}", sample.intent, similarity, sample.weight, score);

        if score > best_score {
            best_score = score;
            best_intent = sample.intent.clone();
        }
    }

    if best_score < 0.5 {
        println!("→ Unsicher (Score {:.3})\n", best_score);
        return Intent::Unknown;
    }

    println!("→ Gewählter Intent: {:?}\n", best_intent);
    best_intent
}
