#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut res = (0..s.len())
            .step_by(((num_rows - 1) * 2) as usize)
            .fold("".to_string(), |a, idx| {
                a + s.get(idx..idx + 1).unwrap()
            });

        for i in 0..num_rows - 2 {
            let step_1 = ((num_rows - 2 - i) * 2) as usize;
            let step_2 = (2 + 2 * i) as usize;

            res += ((i + 1) as usize..s.len())
                .step_by(step_1 + step_2)
                .fold("".to_string(), |a, idx| {
                    a + s.get(idx..idx + 1).unwrap()
                        + s.get(idx + step_1..idx + step_1 + 1).unwrap_or_default()
                })
                .as_str();
        }
        res += ((num_rows - 1) as usize..s.len())
            .step_by(((num_rows - 1) * 2) as usize)
            .fold("".to_string(), |a, idx| {
                a + s.get(idx..idx + 1).unwrap()
            })
            .as_str();

        res
    }
}
// end_submission
