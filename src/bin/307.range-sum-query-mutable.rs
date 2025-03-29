struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0; 2 * n];

        for i in 0..n {
            tree[n + i - 1] = nums[i];
        }

        for i in (0..n - 1).rev() {
            tree[i] = tree[2 * i + 1] + tree[2 * i + 2];
        }

        Self { tree, n }
    }

    fn update(self: &mut Self, index: i32, val: i32) {
        let mut p = self.n + index as usize - 1;

        self.tree[p] = val;

        while p > 0 {
            p = (p - 1) / 2;

            self.tree[p] = self.tree[2 * p + 1] + self.tree[2 * p + 2];
        }
    }

    fn sum_range(self: &Self, left: i32, right: i32) -> i32 {
        let mut res = 0;

        let (mut l_ptr, mut r_ptr) = (left as usize + self.n - 1, right as usize + self.n);

        while l_ptr < r_ptr {
            if l_ptr % 2 == 0 {
                res += self.tree[l_ptr];
                l_ptr += 1;
            }

            if r_ptr % 2 == 0 {
                r_ptr -= 1;
                res += self.tree[r_ptr];
            }

            l_ptr = (l_ptr - 1) / 2;
            r_ptr = (r_ptr - 1) / 2;
        }

        return res;
    }
}
