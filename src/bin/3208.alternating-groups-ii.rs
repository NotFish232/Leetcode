impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut cur_len = 1;
        let mut count = 0;

        for i in 1..colors.len() + k as usize - 1 {
            if colors[i % colors.len()] == colors[(i - 1) % colors.len()] {
                cur_len = 1;
            } else {
                cur_len += 1;
                if cur_len == k {
                    count += 1;
                    cur_len -= 1;
                }
            }
        }

        count as i32
    }
}
