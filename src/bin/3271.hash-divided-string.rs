impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<_>>()
            .chunks(k as usize)
            .map(|x| {
                ('a' as u8 + (x.iter().map(|&c| c as u16 - 'a' as u16).sum::<u16>() % 26) as u8)
                    as char
            })
            .collect()
    }
}
