impl Solution {
    const MOD: i32 = 1_000_000_007;

    fn digit_dp(
        num: &Vec<i32>,
        pos: usize,
        cur_sum: i32,
        is_tight: usize,
        min_sum: i32,
        max_sum: i32,
        dp: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if pos == num.len() {
            return if cur_sum >= min_sum && cur_sum <= max_sum {
                1
            } else {
                0
            };
        }

        if cur_sum > max_sum {
            return 0;
        }

        if dp[pos][cur_sum as usize][is_tight] != -1 {
            return dp[pos][cur_sum as usize][is_tight];
        }

        let ul = if is_tight == 1 { num[pos] } else { 9 };

        let mut res = 0;

        for i in 0..=ul {
            let new_sum = cur_sum + i;
            let new_tight = if is_tight == 0 || i < num[pos] { 0 } else { 1 };
            res += Self::digit_dp(num, pos + 1, new_sum, new_tight, min_sum, max_sum, dp);
            res %= Self::MOD;
        }

        dp[pos][cur_sum as usize][is_tight] = res;

        res
    }

    pub fn solve(num: String, min_sum: i32, max_sum: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; 2]; max_sum as usize + 1]; 23];

        let num = num
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        Self::digit_dp(&num, 0, 0, 1, min_sum, max_sum, &mut dp)
    }

    pub fn count(mut num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let lb_sum = num1.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as i32;

        (Self::MOD + Self::solve(num2, min_sum, max_sum) - Self::solve(num1, min_sum, max_sum)
            + if lb_sum >= min_sum && lb_sum <= max_sum {
                1
            } else {
                0
            })
            % Self::MOD
    }
}
