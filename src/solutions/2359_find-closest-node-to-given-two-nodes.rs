#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::cmp::max;

impl Solution {
    fn node_distances(mut n: i32, edges: &[i32]) -> Vec<usize> {
        let mut dists = vec![usize::MAX; edges.len()];

        let mut dist = 0;
        while n != -1 && dists[n as usize] == usize::MAX {
            dists[n as usize] = dist;
            dist += 1;

            n = edges[n as usize];
        }

        dists
    }

    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let (dist_1, dist_2) = (
            Self::node_distances(node1, &edges),
            Self::node_distances(node2, &edges),
        );

        dist_1
            .into_iter()
            .zip(dist_2)
            .enumerate()
            .filter(|&(_, (d1, d2))| d1 != usize::MAX && d2 != usize::MAX)
            .min_by_key(|&(i, (d1, d2))| (max(d1, d2), i))
            .map_or(-1, |x| x.0 as i32)
    }
}
// end_submission
