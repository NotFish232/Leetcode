#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
impl Solution {
    fn dfs(n: usize, p: usize, adj: &[Vec<usize>], k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut res = 1;

        for &other_n in &adj[n] {
            if other_n != p {
                res += Self::dfs(other_n, n, adj, k - 1);
            }
        }

        res
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

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let (adj_1, adj_2) = (Self::make_adj(&edges1), Self::make_adj(&edges2));

        let max_graph = (0..=edges2.len())
            .map(|n| Self::dfs(n, usize::MAX, &adj_2, k - 1))
            .max()
            .unwrap();

        (0..=edges1.len())
            .map(|n| Self::dfs(n, usize::MAX, &adj_1, k) + max_graph)
            .collect()
    }
}
// end_submission
