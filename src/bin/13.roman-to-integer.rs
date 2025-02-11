impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        let values_and_symbols = [
            (900, "CM"),
            (400, "CD"),
            (90, "XC"),
            (40, "XL"),
            (9, "IX"),
            (4, "IV"),
            (1000, "M"),
            (500, "D"),
            (100, "C"),
            (50, "L"),
            (10, "X"),
            (5, "V"),
            (1, "I"),
        ];
        let mut num = 0;

        while !s.is_empty() {
            for (value, symbol) in values_and_symbols {
                if s.starts_with(symbol) {
                    s = s.strip_prefix(symbol).unwrap().to_string();
                    num += value;
                    break;
                }
            }
        }

        num
    }
}
