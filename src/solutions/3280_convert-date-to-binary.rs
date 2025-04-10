#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|s| format!("{:b}", s.parse::<i32>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}
// end_submission
