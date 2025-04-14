#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count = 0;

        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                for k in j + 1..arr.len() {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
// end_submission
