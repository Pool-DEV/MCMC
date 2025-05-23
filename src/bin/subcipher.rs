use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

fn normalize_text(text: &str) -> String {
    text.nfd()
        .filter(|c| !is_combining_mark(*c))
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn generate_random_key() -> Vec<char> {
    let mut alphabet: Vec<char> = ('A'..='Z').collect();
    let mut rng = thread_rng();
    alphabet.shuffle(&mut rng);
    alphabet
}

fn encrypt(text: &str, key: &[char]) -> String {
    let base_alphabet: Vec<char> = ('A'..='Z').collect();
    let map: HashMap<char, char> = base_alphabet.iter().cloned().zip(key.iter().cloned()).collect();

    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                *map.get(&c).unwrap()
            } else if c.is_ascii_lowercase() {
                let upper = c.to_ascii_uppercase();
                let enc = *map.get(&upper).unwrap();
                enc.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("🔴 ERROR: Usage: {} <plaintext>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    println!("🔵 INFO: Reading file '{}'", filename);

    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("🔴 ERROR: Failed to read file '{}': {}", filename, e);
            std::process::exit(1);
        });

    println!("🔵 INFO: Generating a random key...");
    let key = generate_random_key();

    println!("🔵 INFO: Normalizing text...");
    let normalized = normalize_text(&contents);

    println!("🔵 INFO: Encrypting text...");
    let ciphertext = encrypt(&normalized, &key);

    let assets_dir = Path::new("assets");

    if !assets_dir.exists() {
        println!("🟡 WARNING: 'assets/' directory does not exist. Creating it...");
        fs::create_dir_all(assets_dir).expect("🔴 ERROR: Failed to create 'assets/' directory");
    }

    let mut file = File::create("assets/ciphertext.txt")
        .expect("🔴 ERROR: Failed to create 'assets/ciphertext.txt'");
    file.write_all(ciphertext.as_bytes())
        .expect("🔴 ERROR: Failed to write ciphertext");

    println!("🟢 SUCCESS: Ciphertext written to 'assets/ciphertext.txt'");
    println!("🔑 KEY: {:?}", key);
}