#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    fn gen_number_patterns(
        current: &mut String,
        chars: &mut Vec<char>,
        pattern: &String,
    ) -> Option<String> {
        if current.len() == pattern.len() + 1 {
            return Some(current.to_string());
        }

        let i = current.len();
        for j in 0..chars.len() {
            let ch = chars[j];

            if i == 0
                || (&pattern[i - 1..i] == "I" && ch > current[i - 1..i].chars().next().unwrap())
                || (&pattern[i - 1..i] == "D" && ch < current[i - 1..i].chars().next().unwrap())
            {
                current.push(ch);
                chars.remove(j);

                let res = Solution::gen_number_patterns(current, chars, pattern);

                if res.is_some() {
                    return res;
                }

                current.pop();
                chars.insert(j, ch);
            }
        }

        None
    }

    pub fn smallest_number(pattern: String) -> String {
        Solution::gen_number_patterns(
            &mut String::new(),
            &mut "123456789".chars().collect(),
            &pattern,
        )
        .unwrap()
    }
}
// end_submission
