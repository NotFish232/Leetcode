#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashMap;

struct Encrypter {
    key_to_val: HashMap<char, String>,
    encrypted_to_count: HashMap<String, i32>,
}

impl Encrypter {
    fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let key_to_val = keys.into_iter().zip(values).collect();

        let mut encrypted_to_count = HashMap::new();

        for word in dictionary {
            *encrypted_to_count
                .entry(Self::_encrypt(word, &key_to_val))
                .or_insert(0) += 1;
        }

        Encrypter {
            key_to_val,
            encrypted_to_count,
        }
    }

    fn _encrypt(word: String, key_to_val: &HashMap<char, String>) -> String {
        let s: String = word
            .chars()
            .map(|c| key_to_val.get(&c).unwrap_or(&String::from("!")).clone())
            .collect();

        if !s.contains('!') {
            s
        } else {
            String::new()
        }
    }

    fn encrypt(&self, word1: String) -> String {
        Self::_encrypt(word1, &self.key_to_val)
    }

    fn decrypt(&self, word2: String) -> i32 {
        *self.encrypted_to_count.get(&word2).unwrap_or(&0)
    }
}
// end_submission
