impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut v = vec![true; n as usize];
        let mut count = 0;

        let mut i = 2;
        while i < n as usize {
            if v[i] {
                count += 1;

                let mut j = i * i;
                while j < n as usize {
                    v[j] = false;
                    j += i;
                }
            }

            i += 1;
        }

        count
    }
}
