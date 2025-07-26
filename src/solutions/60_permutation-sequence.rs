#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut nums: Vec<_> = (1..=n).collect();

        k -= 1;

        let mut s = String::new();

        for i in 1..=n {
            let fac = (1..=n - i).product::<i32>();
            let j = k / fac;

            s.push(char::from_digit(nums.remove(j as usize) as u32, 10).unwrap());

            k -= j * fac;
        }

        s
    }
}
// end_submission
