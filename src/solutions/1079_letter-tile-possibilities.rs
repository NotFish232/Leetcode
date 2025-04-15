#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::collections::HashSet;

impl Solution {
    fn _num_tile_possibilities(
        current: &mut String,
        tiles: &mut [i32; 26],
        result: &mut HashSet<String>,
    ) {
        if !current.is_empty() {
            result.insert(current.clone());
        }

        for i in 0..26 {
            if tiles[i] > 0 {
                current.push((i as u8 + b'A') as char);
                tiles[i] -= 1;

                Solution::_num_tile_possibilities(current, tiles, result);

                current.pop();
                tiles[i] += 1;
            }
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut c_tiles = tiles.chars().fold([0; 26], |mut v, ch| {
            v[(ch as u8 - b'A') as usize] += 1;
            v
        });
        let mut result = HashSet::new();

        Solution::_num_tile_possibilities(&mut String::new(), &mut c_tiles, &mut result);

        result.len() as i32
    }
}
// end_submission
