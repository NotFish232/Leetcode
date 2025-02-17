impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        String::from_iter(s.chars().collect::<Vec<_>>().chunks(k as usize).map(|x| {
            ('a' as u128 + (x.iter().map(|&c| c as u128 - 'a' as u128).sum::<u128>() % 26)) as u8
                as char
        }))
    }
}
