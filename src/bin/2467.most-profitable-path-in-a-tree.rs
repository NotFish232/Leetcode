use std::{cmp::max, collections::HashSet};

impl Solution {
    fn find_all_alice_paths(
        current_path: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
        t_map: &Vec<Vec<usize>>,
        seen_idxs: &mut HashSet<usize>,
    ) {
        let mut could_move = false;

        for &idx in t_map[current_path[current_path.len() - 1]].iter() {
            if !seen_idxs.contains(&idx) {
                could_move = true;

                seen_idxs.insert(idx);
                current_path.push(idx);

                Self::find_all_alice_paths(current_path, result, t_map, seen_idxs);

                current_path.pop();
                seen_idxs.remove(&idx);
            }
        }

        if !could_move {
            result.push(current_path.clone())
        }
    }

    pub fn most_profitable_path(mut edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut t_map = vec![Vec::new(); edges.len() + 1];
        for edge in edges {
            t_map[edge[0] as usize].push(edge[1] as usize);
            t_map[edge[1] as usize].push(edge[0] as usize);
        }

        let mut alice_paths = Vec::new();
        Self::find_all_alice_paths(
            &mut vec![0],
            &mut alice_paths,
            &t_map,
            &mut HashSet::from([0]),
        );

        let mut bob_path = Vec::new();
        for alice_path in alice_paths.iter() {
            if let Some(idx) = alice_path.iter().position(|&x| x == bob as usize) {
                bob_path = alice_path[0..idx + 1].iter().rev().map(|&x| x).collect();
            }
        }

        let mut max_score = i32::MIN;

        for alice_path in alice_paths {
            let mut score = 0;
            let mut a_ptr = 0;
            let mut b_ptr = 0;

            let mut bob_seen = HashSet::from([bob_path[b_ptr]]);

            while a_ptr < alice_path.len() {
                if alice_path[a_ptr] == bob_path[b_ptr] {
                    score += amount[alice_path[a_ptr] as usize] / 2;
                } else if !bob_seen.contains(&alice_path[a_ptr]) {
                    score += amount[alice_path[a_ptr] as usize];
                }
                a_ptr += 1;
                if b_ptr + 1 < bob_path.len() {
                    b_ptr += 1;
                    bob_seen.insert(bob_path[b_ptr]);
                }
            }

            max_score = max(max_score, score);
        }

        max_score
    }
}
