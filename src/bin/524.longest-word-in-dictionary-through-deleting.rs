impl Solution {
    pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
        dictionary.sort_by(|a, b| {
            if a.len() != b.len() {
                b.len().cmp(&a.len())
            } else {
                a.cmp(b)
            }
        });

        for word in dictionary {
            let mut i = 0;
            let mut j = 0;

            while i < word.len() && j < s.len() {
                if word[i..i + 1] == s[j..j + 1] {
                    i += 1;
                }
                j += 1;
            }

            if i == word.len() {
                return word;
            }
        }

        String::new()
    }
}
