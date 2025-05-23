use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn load_ciphertext(path: &str) -> String {
    let file = File::open(path).expect("🔴 ERROR: Could not open ciphertext file");
    let reader = BufReader::new(file);
    let mut text = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        text.push_str(&line.to_uppercase());
    }

    text.chars().filter(|c| c.is_ascii_alphabetic()).collect()
}

fn generate_random_key() -> Vec<char> {
    let mut alphabet: Vec<char> = ('A'..='Z').collect();
    let mut rng = thread_rng();
    alphabet.shuffle(&mut rng);
    alphabet
}

fn load_ngram_logprobs(path: &str) -> HashMap<String, f64> {
    let file = File::open(path).expect("🔴 ERROR: Failed to open n-gram file");
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut total: usize = 0;
    let mut counts = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let ngram = parts[0].to_string();
            let count: usize = parts[1].parse().unwrap();
            total += count;
            counts.push((ngram, count));
        }
    }

    for (ngram, count) in counts {
        let prob = count as f64 / total as f64;
        map.insert(ngram, prob.ln());
    }

    map
}

fn apply_key(text: &str, key: &[char]) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let index = (c as u8 - b'A') as usize;
                key[index]
            } else {
                c
            }
        })
        .collect()
}

fn compute_log_likelihood(text: &str, log_probs: &HashMap<String, f64>) -> f64 {
    let chars: Vec<char> = text.chars().collect();
    let mut score = 0.0;

    for i in 0..(chars.len() - 2) {
        let trigram: String = vec![chars[i], chars[i + 1], chars[i + 2]].iter().collect();
        if let Some(prob) = log_probs.get(&trigram) {
            score += prob;
        } else {
            score += (1e-6f64).ln();
        }
    }

    score / text.len() as f64
}

fn permute_key(key: &[char], i: usize, j: usize) -> Vec<char> {
    let mut new_key = key.to_vec();
    new_key.swap(i, j);
    new_key
}

fn main() {
    println!("🔵 INFO: Loading ciphertext...");

    let ciphertext = load_ciphertext("assets/ciphertext.txt");

    println!("🔵 INFO: Loading n-gram probabilities...");

    let trigram_logprobs = load_ngram_logprobs("assets/trigrams.txt");

    println!("🔵 INFO: Generating a random key...");

    let key = generate_random_key();

    println!("🔵 INFO: Applying random key to ciphertext...");

    let decoded = apply_key(&ciphertext, &key);
    let score = compute_log_likelihood(&decoded, &trigram_logprobs);

    println!("🔵 INFO: Initial score: {:.4}", score);

    println!("🔵 INFO: Starting Metropolis algorithm...");

    // TODO
}