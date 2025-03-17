use std::{
    cmp::{Reverse, min},
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    fn is_greater_than_how_many(x: i32, m: i32, n: i32) -> i32 {
        let mut count = 0;

        for i in 1..=m {
            count += min(x / i, n);
        }

        count
    }
    pub fn find_kth_number(m: i32, n: i32, mut k: i32) -> i32 {
        let (mut l, mut r) = (1, m * n);

        while l < r {
            let x = l + (r - l) / 2;

            if Self::is_greater_than_how_many(x, m, n) >= k {
                r = x;
            } else {
                l = x + 1;
            }
        }

        l
    }
}
