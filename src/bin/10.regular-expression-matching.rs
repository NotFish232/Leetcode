use std::cmp::max;

impl Solution {
    fn _is_match(s: &String, p: &String, s_idx: usize, p_idx: usize) -> bool {
        if p_idx == p.len() {
            return s_idx == s.len();
        }

        let char_1 = p.get(p_idx..p_idx + 1);
        let char_2 = p.get(p_idx + 1..p_idx + 2);

        match (char_1, char_2) {
            (Some(a), Some("*")) => {
                (s_idx < s.len()
                    && (a == "." || s.get(s_idx..s_idx + 1) == Some(a))
                    && Solution::_is_match(s, p, s_idx + 1, p_idx))
                    || Solution::_is_match(s, p, s_idx, p_idx + 2)
            }
            (Some(a), _) => {
                if s_idx < s.len() && (a == "." || s.get(s_idx..s_idx + 1) == Some(a)) {
                    Solution::_is_match(s, p, s_idx + 1, p_idx + 1)
                } else {
                    false
                }
            }
            _ => panic!(),
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        Solution::_is_match(&s, &p, 0, 0)
    }
}
