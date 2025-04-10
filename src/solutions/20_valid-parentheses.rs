#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = Vec::new();

        for char in s.chars() {
            match char {
                '(' | '[' | '{' => {
                    v.push(char);
                }
                ')' | ']' | '}' => {
                    let correct = if let Some(c) = v.pop() {
                        char == match c {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            _ => panic!(),
                        }
                    } else {
                        false
                    };
                    if !correct {
                        return false;
                    }
                }
                _ => panic!(),
            }
        }

        v.is_empty()
    }
}
// end_submission
