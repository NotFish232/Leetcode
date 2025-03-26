use std::cmp::max;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut v = Vec::new();

        let m = grid[0][0] % x;

        for r in grid {
            for n in r {
                v.push(n);

                if n % x != m {
                    return -1;
                }
            }
        }

        v.sort();

        let m = v.len() / 2;

        let mut p = vec![m];
        if v.len() % 2 == 0 {
            p.push(m - 1);
        }

        let mut max_c = i32::MIN;

        for i in p {
            let mut c = 0;
            for j in 0..v.len() {
                c += (v[j] - v[i]).abs() / x;
            }

            max_c = max(max_c, c);
        }

        max_c
    }
}
