#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut l_f = vec![0; dominoes.len()];
        let mut r_f = vec![0; dominoes.len()];

        for (i, c) in dominoes.as_bytes().iter().enumerate().rev() {
            match c {
                b'L' => l_f[i] = 1,
                b'.' => {
                    if i + 1 != dominoes.len() && l_f[i + 1] != 0 {
                        l_f[i] = l_f[i + 1] + 1;
                    }
                }
                b'R' => l_f[i] = 0,
                _ => unreachable!(),
            }
        }

        for (i, c) in dominoes.as_bytes().iter().enumerate() {
            match c {
                b'R' => r_f[i] = 1,
                b'.' => {
                    if i != 0 && r_f[i - 1] != 0 {
                        r_f[i] = r_f[i - 1] + 1;
                    }
                }
                b'L' => r_f[i] = 0,
                _ => unreachable!(),
            }
        }

        let mut out = String::new();

        for (&l, &r) in l_f.iter().zip(&r_f) {
            if l == r {
                out.push('.');
            } else if l == 0 || (r != 0 && r < l) {
                out.push('R');
            } else {
                out.push('L');
            }
        }

        out
    }
}
// end_submission
