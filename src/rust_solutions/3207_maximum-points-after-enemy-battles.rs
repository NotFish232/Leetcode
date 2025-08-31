#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, mut current_energy: i32) -> i64 {
        let (min_idx, min_val) = enemy_energies
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, &x)| (i, x))
            .unwrap();

        if current_energy < min_val {
            return 0;
        }

        let mut count = (current_energy / min_val) as i64;
        current_energy %= min_val;

        for (i, e) in enemy_energies.into_iter().enumerate() {
            if i == min_idx {
                continue;
            }
            current_energy += e;
            count += (current_energy / min_val) as i64;
            current_energy %= min_val;
        }

        count
    }
}
// end_submission
