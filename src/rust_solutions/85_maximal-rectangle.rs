#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());

        let mut v = vec![0; m + 1];
        let mut s = Vec::new();

        let mut max_area = -1;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '1' {
                    v[j] += 1;
                } else {
                    v[j] = 0;
                }
            }

            for (j, &h) in v.iter().enumerate() {
                let mut initial_j = j;

                while let Some(&(o_h, k)) = s.last() {
                    if o_h >= h {
                        max_area = max(max_area, (j - k) as i32 * o_h);
                        initial_j = k;
                        s.pop();
                    } else {
                        break;
                    }
                }

                s.push((h, initial_j))
            }
        }

        max_area
    }
}
// end_submission
