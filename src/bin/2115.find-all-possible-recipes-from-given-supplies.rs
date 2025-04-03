#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn find_all_recipes(
        mut recipes: Vec<String>,
        mut ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut current: HashSet<_> = supplies.into_iter().collect();
        let mut made = Vec::new();

        loop {
            let mut found_idx = -1;

            for (idx, i) in ingredients.iter().enumerate() {
                if i.iter().all(|x| current.contains(x)) {
                    found_idx = idx as i32;
                    break;
                }
            }

            if found_idx != -1 {
                let found_idx = found_idx as usize;

                current.insert(recipes[found_idx].clone());
                made.push(recipes[found_idx].clone());

                recipes.remove(found_idx);
                ingredients.remove(found_idx);
            } else {
                break;
            }
        }

        made
    }
}
// end_submission
