use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::new();

        for num in nums {
            h.push(num);
        }

        for _ in 0..(k - 1) as usize {
            h.pop();
        }

        h.pop().unwrap()
    }
}
