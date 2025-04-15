struct DisjointSetUnion {
    parents: Vec<usize>,
    sizes: Vec<i32>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        if self.parents[idx] != idx {
            self.parents[idx] = self.find(self.parents[idx]);
        }

        self.parents[idx]
    }

    fn union(&mut self, idx_1: usize, idx_2: usize) {
        let rep_1 = self.find(idx_1);
        let rep_2 = self.find(idx_2);

        if rep_1 != rep_2 {
            if self.sizes[rep_1] < self.sizes[rep_2] {
                self.parents[rep_1] = rep_2;
                self.sizes[rep_2] += self.sizes[rep_1];
            } else {
                self.parents[rep_2] = rep_1;
                self.sizes[rep_1] += self.sizes[rep_2];
            }
        }
    }
}
