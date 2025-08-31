#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn gen_happy_strings(
        current: &mut String,
        result: &mut Vec<String>,
        n: i32,
        num_req: &mut i32,
    ) {
        if *num_req == 0 {
            return;
        }

        if n == 0 {
            result.push(current.to_string());
            *num_req -= 1;
            return;
        }

        for ch in ["a", "b", "c"] {
            if current.is_empty() || current.get(current.len() - 1..) != Some(ch) {
                current.push_str(ch);

                Solution::gen_happy_strings(current, result, n - 1, num_req);

                current.pop();
            }
        }
    }

    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut result = Vec::new();
        Solution::gen_happy_strings(&mut String::new(), &mut result, n, &mut k.clone());

        match result.get(k as usize - 1) {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }
}
// end_submission
