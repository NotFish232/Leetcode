use std::collections::HashMap;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut seen = [0; 3];

        let mut left_ptr = 0;
        let mut right_ptr = 0;

        let mut count = 0;

        while right_ptr < s.len() {
            if let ch @ (b'a' | b'b' | b'c') = s.as_bytes()[right_ptr] {
                seen[(ch - b'a') as usize] += 1
            }

            while seen[0] > 0 && seen[1] > 0 && seen[2] > 0 {
                count += (s.len() - right_ptr) as i32;

                if let ch @ (b'a' | b'b' | b'c') = s.as_bytes()[left_ptr] {
                    seen[(ch - b'a') as usize] -= 1
                }

                left_ptr += 1;
            }

            right_ptr += 1;
        }

        count
    }
}
