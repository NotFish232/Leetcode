#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
        for i in 1..arr.len() {
            arr[i] = (arr[i] + arr[i - 1]) % 2;
        }

        let mut num_even_sums = 1;
        let mut num_odd_sums = 0;
        let mut count = 0;
        let modulo = 10i32.pow(9) + 7;

        for num in arr {
            if num % 2 == 0 {
                count = (count + num_odd_sums) % modulo;
                num_even_sums += 1;
            } else {
                count = (count + num_even_sums) % modulo;
                num_odd_sums += 1;
            }
        }

        count
    }
}
// end_submission
