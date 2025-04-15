#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = vec![false; grid.len() * grid[0].len()];
        let mut a = -1;

        for row in grid {
            for x in row {
                let i = x as usize - 1;
                if v[i] {
                    a = x;
                } else {
                    v[i] = true;
                }
            }
        }

        let mut b = -1;
        for (i, x) in v.into_iter().enumerate() {
            if !x {
                b = (i + 1) as i32;
            }
        }

        vec![a, b]
    }
}
// end_submission
