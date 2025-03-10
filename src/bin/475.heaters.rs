use std::cmp::{max, min};

impl Solution {
    pub fn find_radius(houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        // for each house minus the closest heater
        // how to calculate the closest efficiently
        // houses - max(heaters) / houses - min(heaters) ?

        heaters.sort();

        let mut result = 0;

        for house in &houses {
            match heaters.binary_search(house) {
                Ok(_) => {}
                Err(i) => {
                    let right_val = heaters.get(i).unwrap_or(&i32::MAX);
                    let left_val = heaters.get(i - 1).unwrap_or(&i32::MAX);

                    result = max(
                        result,
                        min((house - *left_val).abs(), (house - *right_val).abs()),
                    );
                }
            }
        }

        result
    }
}
