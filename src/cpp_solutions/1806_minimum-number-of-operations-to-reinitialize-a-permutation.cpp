#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int mod_pow(int base, int exp, int mod) {
        int ans = 1;

        while (exp > 0) {
            if (exp % 2 == 1) {
                ans = (ans * base) % mod;
            }

            base = (base * base) % mod;
            exp /= 2;
        }

        return ans;
    }

    int reinitializePermutation(int n) {
        if (n == 2) {
            return 1;
        }

        int i = 1;

        while (mod_pow(2, i, n - 1) != 1) {
            i++;
        }

        return i;
    }
};
// end_submission
