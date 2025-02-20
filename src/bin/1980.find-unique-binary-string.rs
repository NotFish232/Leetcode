impl Solution {
    pub fn find_different_binary_string(mut nums: Vec<String>) -> String {
        let n = nums[0].len();

        nums.sort();

        for i in 0..2_usize.pow(n as u32) {
            let s = (0..n).rev().map(|x| char::from_digit(((i >> x) & 1) as u32, 10).unwrap()).collect();

            if i >= nums.len() || s != nums[i] {
                return s;
            }
        }

        String::new()
    }   
}
