#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use itertools::repeat_n;
use std::collections::HashMap;

impl Solution {
    const MOD: i32 = 1_000_000_007;

    fn gen_masks(m: i32) -> Vec<(i32, Vec<i32>)> {
        let mut masks = Vec::new();

        'out: for x in 0..3i32.pow(m as u32) {
            let mut colors = Vec::new();

            let mut cur = x;
            while cur > 0 {
                colors.push(cur % 3);
                cur /= 3;
            }
            colors.extend(repeat_n(0, m as usize - colors.len()));

            for (c1, c2) in colors.iter().zip(colors.iter().skip(1)) {
                if c1 == c2 {
                    continue 'out;
                }
            }

            masks.push((x, colors))
        }

        masks
    }
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let masks = Self::gen_masks(m);

        let mut adj = HashMap::new();

        for (m1, c1) in &masks {
            for (m2, c2) in &masks {
                if c1.iter().zip(c2).all(|(x1, x2)| x1 != x2) {
                    adj.entry(m1).or_insert(Vec::new()).push(m2);
                }
            }
        }

        let mut h = HashMap::new();
        for (m, _) in &masks {
            h.insert(m, 1);
        }

        for _ in 1..n {
            let mut new_h = HashMap::new();

            for (m, _) in &masks {
                let mut total = 0;

                if let Some(m_adj) = adj.get(&m) {
                    for o_m in m_adj {
                        total = (total + h.get(o_m).unwrap_or(&0)) % Self::MOD;
                    }
                }

                new_h.insert(m, total);
            }

            h = new_h;
        }

        h.values().fold(0, |acc, x| (acc + x) % Self::MOD)
    }
}
// end_submission
