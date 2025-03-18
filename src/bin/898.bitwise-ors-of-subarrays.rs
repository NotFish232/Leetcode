use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut a = HashSet::new();
        let mut cur = HashSet::new();
        cur.insert(0);

        for n in arr {
            let mut new_cur = HashSet::new();
            for old in cur {
                new_cur.insert(old | n);
            }
            new_cur.insert(n);

            a.extend(new_cur.iter().map(|&x| x));

            cur = new_cur;
        }

        a.len() as i32
    }
}
