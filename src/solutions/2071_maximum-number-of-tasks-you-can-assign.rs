#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{
    cmp::min,
    collections::{btree_map::Entry, BTreeMap},
};

impl Solution {
    fn check(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32, k: usize) -> bool {
        let mut m = BTreeMap::new();

        for &w in workers.iter().rev().take(k) {
            *m.entry(w).or_insert(0) += 1;
        }

        for &t in tasks.iter().take(k).rev() {
            if let Some((&max_key, _)) = m.iter().next_back() {
                if max_key >= t {
                    let Entry::Occupied(mut e) = m.entry(max_key) else {
                        unreachable!()
                    };
                    *e.get_mut() -= 1;
                    if *e.get() == 0 {
                        e.remove_entry();
                    }
                } else {
                    if pills == 0 {
                        return false;
                    }
                    if let Some((&key, _)) = m.range(t - strength..).next() {
                        let Entry::Occupied(mut e) = m.entry(key) else {
                            unreachable!()
                        };
                        *e.get_mut() -= 1;
                        if *e.get() == 0 {
                            e.remove_entry();
                        }
                        pills -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort();
        workers.sort();

        let n = tasks.len();
        let m = workers.len();

        let (mut l, mut r, mut ans) = (1, min(m, n), 0);

        while l <= r {
            let mid = l + (r - l) / 2;

            if Self::check(&tasks, &workers, pills, strength, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        ans as i32
    }
}
// end_submission
