use std::cmp::{max, min};

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut num: i64 = 0;
        let mut is_negative = false;
        let mut seen_non_digit = false;
        for (i, ch) in s.chars().enumerate() {
            if !ch.is_numeric() {
                if !seen_non_digit && (ch == ' ' || ch == '+' || ch == '-') {
                    if ch == '+' || ch == '-' {
                        is_negative = ch == '-';
                        seen_non_digit = true;
                    }
                    continue;
                }
                break;
            }
            if ch != '0' || num != 0 {
                num *= 10;
                num += ch.to_digit(10).unwrap() as i64;
                if num > i32::MAX as i64 {
                    return if is_negative { i32::MIN } else { i32::MAX };
                }
                seen_non_digit = true;
            }
            if ch == '0' {
                seen_non_digit = true;
            }
        }
        num as i32 * if is_negative { -1 } else { 1 } 
    }
}
