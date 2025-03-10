use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();

        for str in strs {
            let mut s_chars: Vec<_> = str.chars().collect();
            s_chars.sort();

            hm.entry(s_chars).or_insert(vec![]).push(str);
        }

        hm.values().map(|x| x.clone()).collect()
    }
}
