#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (has_first_row, has_first_col) = (
            (0..matrix[0].len()).any(|i| matrix[0][i] == 0),
            (0..matrix.len()).any(|i| matrix[i][0] == 0),
        );

        for i in 1..matrix.len() {
            for j in 1..matrix[i].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..matrix.len() {
            for j in 1..matrix[i].len() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if has_first_row {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
        if has_first_col {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
    }
}
// end_submission
