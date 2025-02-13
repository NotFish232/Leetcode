impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut v = vec![0; num1.len() * num2.len()];

        for (i, s_d1) in num1.chars().rev().enumerate() {
            let d1 = s_d1.to_digit(10).unwrap() as u128;

            for (j, s_d2) in num2.chars().rev().enumerate() {
                let d2 = s_d2.to_digit(10).unwrap() as u128;

                v[i + j] += d1 * d2;
            }
        }

        for i in 0..v.len() - 1 {
            v[i + 1] += v[i] / 10;
            v[i] = v[i] % 10;
        }

        if v.iter().any(|&x| x != 0) {
            v.into_iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("")
                .trim_start_matches("0")
                .to_string()
        } else {
            "0".to_string()
        }
    }
}
