#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
struct ProductOfNumbers {
    v: Vec<i32>,
    prod: i32,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            v: Vec::new(),
            prod: 1,
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.v.clear();
            self.prod = 1;
        } else {
            self.prod *= num;
            self.v.push(self.prod);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        if self.v.len() < k as usize {
            0
        } else if self.v.len() == k as usize {
            self.v[self.v.len() - 1]
        } else {
            self.v[self.v.len() - 1] / self.v[self.v.len() - 1 - k as usize]
        }
    }
}
// end_submission
