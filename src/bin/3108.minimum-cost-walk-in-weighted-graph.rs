struct DisjointSetUnion {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> DisjointSetUnion {
        DisjointSetUnion {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn find(self: &mut Self, i: usize) -> usize {
        if i != self.parents[i] {
            self.parents[i] = Self::find(self, self.parents[i]);
            self.parents[i]
        } else {
            i
        }
    }

    fn union(self: &mut Self, a: usize, b: usize, weight: i32) {
        let a_rep = Self::find(self, a);
        let b_rep = Self::find(self, b);

        if a_rep != b_rep {
            if self.ranks[a_rep] < self.ranks[b_rep] {
                self.parents[a_rep] = b_rep;
            } else if self.ranks[b_rep] < self.ranks[a_rep] {
                self.parents[b_rep] = a_rep;
            } else {
                self.parents[a_rep] = b_rep;
                self.ranks[b_rep] += 1;
            }
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DisjointSetUnion::new(n as usize);

        for edge in edges.iter() {
            dsu.union(edge[0] as usize, edge[1] as usize, edge[2]);
        }

        let mut weights = vec![i32::MAX; n as usize];

        for edge in edges.iter() {
            weights[dsu.find(edge[0] as usize)] &= edge[2];
        }

        let mut v = Vec::new();

        for q in query {
            let a_rep = dsu.find(q[0] as usize);
            let b_rep = dsu.find(q[1] as usize);

            if a_rep == b_rep {
                v.push(weights[a_rep]);
            } else {
                v.push(-1);
            }
        }

        v
    }
}
