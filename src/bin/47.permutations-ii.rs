impl Solution {
    fn _create_unique_permutations(
        current: &mut Vec<i32>,
        nums: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if nums.is_empty() {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let num = nums.remove(i);
            current.push(num);

            Self::_create_unique_permutations(current, nums, result);

            nums.insert(i, num);
            current.pop();
        }
    }
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result = Vec::new();

        Self::_create_unique_permutations(&mut Vec::new(), &mut nums, &mut result);

        result
    }
}
