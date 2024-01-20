use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = HashMap::new();
        let mut start = 0;
        let mut longest_len = 0;
        for (i, ch) in s.chars().enumerate() {
            if let Some(&ch_loc) = m.get(&ch) {
                if ch_loc >= start {
                    start = ch_loc + 1;
                }
            }
            let sub_len = i - start + 1;
            if (sub_len > longest_len) {
                longest_len = sub_len;
            }
            m.insert(ch, i);
        }
        longest_len as i32
    }
}
