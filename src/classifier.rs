use crate::intent::Intent;
use crate::similarity::cosine_similarity;
use crate::boost::intent_boost;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IntentSample {
    pub intent: Intent,
    pub vector: Vec<f32>,
    pub weight: f32,
}

pub fn classify(
    input_vec: &[f32],
    original_text: &str,
    samples: &[IntentSample],
) -> Intent {
    let mut best_score = -1.0;
    let mut best_intent = Intent::Unknown;

    for s in samples {
        let sim = cosine_similarity(input_vec, &s.vector);
        let boost = intent_boost(&s.intent, original_text);
        let score = sim * s.weight * boost;

        //Debug Ausgabe:
        //println!("Ähnlichkeit zu {:?}: sim={:.3}, weight={:.2}, boost={:.2}, score={:.3}", s.intent, sim, s.weight, boost, score);

        if score > best_score {
            best_score = score;
            best_intent = s.intent.clone();
        }
    }

    if best_score < 0.5 {
        println!("→ Unsicher (Score {:.3})\n", best_score);
        return Intent::Unknown;
    }

    println!("→ Gewählter Intent: {:?}\n", best_intent);
    best_intent
}
