#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols_and_values = [
            ("M", 1000),
            ("D", 500),
            ("C", 100),
            ("L", 50),
            ("X", 10),
            ("V", 5),
            ("I", 1),
        ];
        let mut res = Vec::new();

        while num > 0 {
            match num.to_string().get(0..1) {
                Some("4") | Some("9") => {
                    for (i, (symbol, value)) in symbols_and_values.iter().enumerate().rev() {
                        if *value > num {
                            let offset = if num.to_string().get(0..1) == Some("4") {
                                1
                            } else {
                                2
                            };
                            res.push(symbols_and_values[i + offset].0.to_string());
                            res.push(symbol.to_string());
                            num -= value - symbols_and_values[i + offset].1;
                            break;
                        }
                    }
                }
                Some(_) => {
                    for (symbol, value) in symbols_and_values {
                        if value <= num {
                            res.push(symbol.to_string());
                            num -= value;
                            break;
                        }
                    }
                }
                None => panic!(),
            }
        }

        res.join("")
    }
}
// end_submission
