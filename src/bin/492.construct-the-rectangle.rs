impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        for i in (area as f32).sqrt().ceil() as i32..=area {
            if area % i == 0 {
                return vec![i, area / i];
            }
        }

        unreachable!()
    }
}
