impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut res = (0..s.len())
            .step_by(((num_rows - 1) * 2) as usize)
            .fold("".to_string(), |a, idx| {
                a + s.get(idx as usize..(idx + 1) as usize).unwrap()
            });

        for i in (0..num_rows - 2) {
            let step_1 = ((num_rows - 2 - i) * 2) as usize;
            let step_2 = (2 + 2 * i) as usize;

            res += ((i + 1) as usize..s.len())
                .step_by(step_1 + step_2)
                .fold("".to_string(), |a, idx| {
                    a + s.get(idx as usize..(idx + 1) as usize).unwrap()
                        + match s.get(idx + step_1..idx + step_1 + 1) {
                            Some(ch) => ch,
                            None => "",
                        }
                })
                .as_str();
        }
        res += ((num_rows - 1) as usize..s.len())
            .step_by(((num_rows - 1) * 2) as usize)
            .fold("".to_string(), |a, idx| {
                a + s.get(idx as usize..(idx + 1) as usize).unwrap()
            })
            .as_str();

        res
    }
}
