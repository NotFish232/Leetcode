use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

struct DSU {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> DSU {
        DSU {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn find(self: &mut Self, i: usize) -> usize {
        if self.parents[i] != i {
            self.parents[i] = self.find(self.parents[i]);
        }

        self.parents[i]
    }

    fn union(self: &mut Self, a: usize, b: usize) {
        let a_rep = self.find(a);
        let b_rep = self.find(b);

        if a_rep != b_rep {
            match self.ranks[a_rep].cmp(&self.ranks[b_rep]) {
                Ordering::Less => self.parents[a_rep] = b_rep,
                Ordering::Greater => self.parents[b_rep] = a_rep,
                Ordering::Equal => {
                    self.parents[b_rep] = a_rep;
                    self.ranks[a_rep] += 1;
                }
            }
        }
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut dsu = DSU::new(n as usize);
        let mut seen_edges = HashSet::new();

        for edge in edges {
            dsu.union(edge[0] as usize, edge[1] as usize);
            seen_edges.insert((edge[0] as usize, edge[1] as usize));
        }

        let mut components = HashMap::new();

        for i in 0..n as usize {
            components.entry(dsu.find(i)).or_insert(vec![]).push(i);
        }

        let mut count = 0;
        for comp in components.values() {
            let mut all_edges = true;
            'out: for i in 1..comp.len() {
                for j in 0..i {
                    if !(seen_edges.contains(&(comp[i], comp[j]))
                        || seen_edges.contains(&(comp[j], comp[i])))
                    {
                        all_edges = false;
                        break 'out;
                    }
                }
            }

            if all_edges {
                count += 1;
            }
        }

        count
    }
}
