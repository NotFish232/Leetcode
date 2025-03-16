impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let (mut l, mut r) = (
            1i64,
            (cars as i64).pow(2) * *ranks.iter().min().unwrap() as i64,
        );

        while l < r {
            let m = l + (r - l) / 2;

            let mut c = 0;
            for &r in &ranks {
                c += (m as f64 / r as f64).sqrt() as i64;
            }

            if c >= cars as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}
