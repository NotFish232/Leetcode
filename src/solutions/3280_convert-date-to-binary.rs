#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|s| format!("{:b}", s.parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}
// end_submission
