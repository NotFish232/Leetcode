#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut x_intervals = Vec::new();
        let mut y_intervals = Vec::new();

        for rect in rectangles {
            let &[x1, y1, x2, y2] = &rect[..] else {
                panic!();
            };

            x_intervals.push(vec![x1, x2]);
            y_intervals.push(vec![y1, y2]);
        }

        x_intervals.sort();
        y_intervals.sort();

        let mut merged_x_intervals: Vec<Vec<i32>> = Vec::new();
        let mut merged_y_intervals: Vec<Vec<i32>> = Vec::new();

        for int in x_intervals {
            let idx = merged_x_intervals.len() - 1;
            if merged_x_intervals.is_empty() || merged_x_intervals[idx][1] <= int[0] {
                merged_x_intervals.push(int);
            } else {
                merged_x_intervals[idx][1] = max(merged_x_intervals[idx][1], int[1]);
            }
        }

        for int in y_intervals {
            let idx = merged_y_intervals.len() - 1;
            if merged_y_intervals.is_empty() || merged_y_intervals[idx][1] <= int[0] {
                merged_y_intervals.push(int);
            } else {
                merged_y_intervals[idx][1] = max(merged_y_intervals[idx][1], int[1]);
            }
        }

        merged_x_intervals.len() >= 3 || merged_y_intervals.len() >= 3
    }
}
// end_submission
