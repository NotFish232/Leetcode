use std::collections::{HashMap, VecDeque};

impl Solution {
    fn bit_encode(mat: &Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();

        let mut b = 0;

        for (i, v) in mat.iter().enumerate() {
            for (j, &x) in v.iter().enumerate() {
                b |= (1 << (i * n + j)) * x;
            }
        }

        b
    }

    fn gen_masks(m: i32, n: i32) -> Vec<i32> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut masks = Vec::new();

        for i in 0..m {
            for j in 0..n {
                let mut b = 1 << (i * n + j);
                for (k, l) in directions {
                    let new_i = (i as i32 + k);
                    let new_j = (j as i32 + l);

                    if 0 <= new_i && new_i < m && 0 <= new_j && new_j < n {
                        b |= 1 << (new_i * n +  new_j);
                    }
                }

                masks.push(b);
            }
        }

        masks
    }

    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let mat_bits = Solution::bit_encode(&mat);
        let masks = Solution::gen_masks(mat.len() as i32, mat[0].len() as i32);

        let mut h = HashMap::from([(0, 0)]);
        let mut q = VecDeque::from([0]);

        while !q.is_empty() {
            let x = q.pop_front().unwrap();
            let x_dist = *h.get(&x).unwrap();

            for &mask in masks.iter() {
                let b = x ^ mask;

                if !h.contains_key(&b) {
                    h.insert(b, x_dist + 1);
                    q.push_back(b);
                }
            }

            if h.contains_key(&mat_bits) {
                return *h.get(&mat_bits).unwrap();
            }
        }

        -1
    }
}
