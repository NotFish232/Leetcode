impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut current_max = *nums.last().unwrap() as i64;
        let mut count = 0i64;
        
        for num in nums.into_iter().rev().skip(1).map(|n| n as i64) {
            let num_times = (num + current_max - 1) / current_max;
            count += num_times - 1;
            current_max = num / num_times;
        }
        
        count
    }
}
