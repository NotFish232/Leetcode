impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut dp = vec![vec![false; stones.len() + 1]; stones.len()];
        dp[0][1] = true;

        for i in 1..stones.len() {
            for j in 0..i {
                let k = (stones[i] - stones[j]) as usize;

                if k <= stones.len() && dp[j][k] {
                    dp[i][k] = true;

                    if k > 0 {
                        dp[i][k - 1] = true;
                    }
                    if k < stones.len() {
                        dp[i][k + 1] = true;
                    }
                }
            }
        }

        dp[stones.len() - 1].iter().any(|&x| x)
    }
}
