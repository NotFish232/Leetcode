impl Solution {
    fn num_pairs_less(nums: &Vec<i32>, dist: i32) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut count = 0;

        while r < nums.len() {
            while nums[r] - nums[l] > dist {
                l += 1;
            }

            count += (r - l) as i32;

            r += 1;
        }

        count
    }

    pub fn smallest_distance_pair(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();

        let (mut l, mut r) = (0, nums[nums.len() - 1] - nums[0]);

        while l < r {
            let m = l + (r - l) / 2;

            if Self::num_pairs_less(&nums, m) >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}
