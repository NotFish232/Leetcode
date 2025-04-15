#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut road_edges = vec![Vec::new(); n];

        for r in roads {
            road_edges[r[0] as usize].push((r[1] as usize, r[2] as i64));
            road_edges[r[1] as usize].push((r[0] as usize, r[2] as i64));
        }

        let mut queue = BinaryHeap::new();
        let mut times = vec![i64::MAX; n];
        let mut num_paths = vec![0; n];

        queue.push((Reverse(0), 0));
        times[0] = 0;
        num_paths[0] = 1;

        while let Some((Reverse(time), node)) = queue.pop() {
            if time > times[node] {
                continue;
            }

            for &(nbr, weight) in road_edges[node].iter() {
                if time + weight < times[nbr] {
                    queue.push((Reverse(time + weight), nbr));
                    times[nbr] = time + weight;
                    num_paths[nbr] = num_paths[node];
                } else if time + weight == times[nbr] {
                    num_paths[nbr] += num_paths[node];
                    num_paths[nbr] %= Self::MOD;
                }
            }
        }

        num_paths[n - 1] as i32
    }
}
// end_submission
