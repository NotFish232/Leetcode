impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s_copy = s.to_string();
        
        loop {
            if let Some(idx) = s_copy.find(part.as_str()) {
                for _ in 0..part.len() {
                    s_copy.remove(idx);
                }
            } else {
                break;
            }
        }

        s_copy
    }
}
