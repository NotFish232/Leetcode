impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut pairs = Vec::new();
        for i in 0..weights.len() - 1 {
            pairs.push(weights[i] + weights[i + 1]);
        }
        pairs.sort();

        let mut count = 0;

        for i in 0..k as usize - 1 {
            count += (pairs[pairs.len() - i - 1] - pairs[i]) as i64;
        }

        count
    }
}
