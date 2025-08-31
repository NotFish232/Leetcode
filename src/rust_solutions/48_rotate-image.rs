#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let n = matrix.len();

        for i in 0..n {
            for j in 0..i + 1 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for row in matrix {
            for j in 0..n / 2 {
                row.swap(j, n - j - 1);
            }
        }
    }
}
// end_submission
