impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix_chars = Vec::new();

        'out: for i in 0..strs[0].len() {
            prefix_chars.push(strs[0].get(i..i + 1).unwrap());
            for str in &strs {
                if str.get(i..i + 1) != Some(prefix_chars.last().unwrap()) {
                    prefix_chars.pop();
                    break 'out;
                }
            }
        }

        prefix_chars.join("")
    }
}
