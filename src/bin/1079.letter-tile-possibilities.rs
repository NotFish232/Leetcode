use std::collections::{HashMap, HashSet};

impl Solution {
    fn _num_tile_possibilities(
        current: &String,
        tiles: &mut Vec<String>,
        result: &mut HashSet<String>,
    ) {
        if current.len() != 0 {
            result.insert(current.to_string());
        }

        for i in 0..tiles.len() {
            let tile = tiles.remove(i);
            let new_current = current.to_string() + tile.as_str();

            Solution::_num_tile_possibilities(&new_current, tiles, result);

            tiles.insert(i, tile);
        }
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut tiles_v = tiles.chars().map(|c| c.to_string()).collect();
        let mut result = HashSet::new();

        Solution::_num_tile_possibilities(&String::new(), &mut tiles_v, &mut result);

        result.len() as i32
    }
}
