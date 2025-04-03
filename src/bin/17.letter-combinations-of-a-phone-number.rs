#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let digit_to_letters = HashMap::from([
            ("2", "abc"),
            ("3", "def"),
            ("4", "ghi"),
            ("5", "jkl"),
            ("6", "mno"),
            ("7", "pqrs"),
            ("8", "tuv"),
            ("9", "wxyz"),
        ]);

        let letters = digit_to_letters.get(&digits.get(0..1).unwrap()).unwrap();

        if digits.len() == 1 {
            return letters.chars().map(|c| c.to_string()).collect();
        }

        let mut combinations = Vec::new();

        let future_combinations =
            Solution::letter_combinations(digits.get(1..digits.len()).unwrap().to_string());

        for letter in letters.chars() {
            for future_combo in &future_combinations {
                let mut owned_letter = letter.to_string();
                owned_letter.push_str(future_combo.as_str());
                combinations.push(owned_letter);
            }
        }

        combinations
    }
}
// end_submission
