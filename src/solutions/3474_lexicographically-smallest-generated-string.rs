#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn make_lps(str: &str) -> Vec<usize> {
        let str_chars: Vec<_> = str.chars().collect();

        let mut lps = vec![0; str_chars.len()];
        lps[0] = 0;

        let mut i = 1;
        let mut len = 0;

        while i < str_chars.len() {
            if str_chars[i] == str_chars[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                i += 1;
            }
        }

        lps
    }

    pub fn generate_string(str1: String, str2: String) -> String {
        let lps = Self::make_lps(&str2);

        let chars_1: Vec<_> = str1.chars().collect();
        let chars_2: Vec<_> = str2.chars().collect();

        let mut v = vec![' '; str1.len() + str2.len() - 1];

        for i in 0..chars_1.len() {
            if chars_1[i] == 'T' {
                for j in 0..chars_2.len() {
                    if v[i + j] != ' ' && v[i + j] != chars_2[j] {
                        return String::new();
                    }

                    v[i + j] = chars_2[j];
                }
            }
        }

        let mut j = 0;
        let mut i = 0;
        while i < v.len() {
            if v[i] == chars_2[j] {
                i += 1;
                j += 1;

                if j == chars_2.len() {
                    if chars_1[i - j] == 'F' {
                        return String::new();
                    }
                    j = lps[j - 1];
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }

        for i in 0..v.len() {
            if v[i] == ' ' {
                v[i] = 'a';
            }
        }

        let mut j = 0;
        let mut i = 0;
        while i < v.len() {
            if v[i] == chars_2[j] {
                i += 1;
                j += 1;

                if j == chars_2.len() {
                    if chars_1[i - j] == 'F' {
                        for k in 0..chars_2.len() {
                            if v[i - k - 1] == 'a' {
                                v[i - k - 1] = 'b';
                                break;
                            }
                        }
                        j = 0;
                    } else {
                        j = lps[j - 1];
                    }
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }

        v.into_iter().collect()
    }
}
// end_submission
