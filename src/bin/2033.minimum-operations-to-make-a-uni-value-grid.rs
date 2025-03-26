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

        let mut c = 0;
        for i in 0..v.len() {
            c += (v[i] - v[m]).abs() / x;
        }

        c
    }
}
