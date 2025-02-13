impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(x) => x as i32,
            Err(_) => nums.partition_point(|&x| x < target) as i32,
        }
    }
}
