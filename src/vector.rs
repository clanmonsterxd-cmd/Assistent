use std::collections::HashMap;
use crate::normalize::normalize;

pub struct Vocab {
    vectors: HashMap<String, Vec<f32>>,
    dim: usize,
}

impl Vocab {
    pub fn new(dim: usize) -> Self {
        Self { vectors: HashMap::new(), dim }
    }

    fn random_vec(&self, word: &str) -> Vec<f32> {
        let mut hash: u32 = 2166136261;
        for b in word.bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(16777619);
        }

        let mut seed = hash;
        (0..self.dim).map(|_| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed as f32 / u32::MAX as f32) - 0.5
        }).collect()
    }

    pub fn word_vec(&mut self, word: &str) -> Vec<f32> {
        if let Some(v) = self.vectors.get(word) {
            return v.clone();
        }
        let v = self.random_vec(word);
        self.vectors.insert(word.to_string(), v.clone());
        v
    }

    pub fn sentence_vec(&mut self, text: &str) -> Vec<f32> {
        let norm = normalize(text);
        let words: Vec<&str> = norm.split_whitespace().collect();

        let mut sum = vec![0.0; self.dim];
        for w in &words {
            let v = self.word_vec(w);
            for i in 0..self.dim {
                sum[i] += v[i];
            }
        }

        for i in 0..self.dim {
            sum[i] /= words.len() as f32;
        }

        sum
    }
}
