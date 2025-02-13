use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, mut words: Vec<String>) -> Vec<i32> {
        let mut words_s = words.join("");
        let mut words_m: HashMap<String, i32> = HashMap::new();
        for word in &words {
            *words_m.entry(word.to_string()).or_default() += 1
        }

        s.chars()
            .collect::<Vec<_>>()
            .windows(words.iter().map(|w| w.len()).sum())
            .enumerate()
            .filter(|(_, c)| {
                if c.iter().collect::<String>() == words_s {
                    return true;
                }

                let mut m: HashMap<String, _> = HashMap::new();
                for x in c.chunks(words[0].len()) {
                    *m.entry(x.iter().collect()).or_default() += 1;
                }

                m == words_m
            })
            .map(|(i, _)| i as i32)
            .collect()
    }
}
