impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        (((1 << ((num as f64).log2() as i64 + 1)) - 1) ^ num as i64) as i32
    }
}
