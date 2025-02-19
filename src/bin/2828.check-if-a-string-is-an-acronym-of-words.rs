impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words
            .into_iter()
            .map(|w| w.chars().next().unwrap())
            .eq(s.chars())
    }
}
