impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        blocks
            .chars()
            .collect::<Vec<_>>()
            .windows(k as usize)
            .map(|c| c.iter().filter(|&&x| x == 'W').count())
            .min()
            .unwrap() as i32
    }
}
