#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{array, cmp::max, collections::HashSet};

impl Solution {
    fn dfs(n: usize, p: usize, adj: &[Vec<usize>], d: i32, res: &mut [HashSet<usize>; 2]) {
        res[(d % 2 == 0) as usize].insert(n);

        for &other_n in &adj[n] {
            if other_n != p {
                Self::dfs(other_n, n, adj, d + 1, res);
            }
        }
    }

    fn make_adj(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::new(); edges.len() + 1];

        for e in edges {
            let (e_1, e_2) = (e[0] as usize, e[1] as usize);

            adj[e_1].push(e_2);
            adj[e_2].push(e_1);
        }

        adj
    }

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (adj_1, adj_2) = (Self::make_adj(&edges1), Self::make_adj(&edges2));

        let (mut res_1, mut res_2) = (
            array::from_fn(|_| HashSet::new()),
            array::from_fn(|_| HashSet::new()),
        );

        Self::dfs(0, usize::MAX, &adj_1, 0, &mut res_1);
        Self::dfs(0, usize::MAX, &adj_2, 1, &mut res_2);

        (0..=edges1.len())
            .map(|n| {
                ((if res_1[0].contains(&n) {
                    res_1[0].len()
                } else {
                    res_1[1].len()
                }) + max(res_2[0].len(), res_2[1].len())) as i32
            })
            .collect()
    }
}
// end_submission
