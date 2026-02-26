#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int minOperations(int n) {
        int one_count = 0;
        int ans = 0;

        while (n > 0) {
            int x = n & 1;

            one_count += x;

            if (x == 0 && one_count > 0) {
                if (one_count == 1) {
                    ans++;
                } else {
                    n += 1;
                    ans++;
                }

                one_count = 0;
            } else {
                n >>= 1;
            }
        }

        if (one_count > 1) {
            ans += 2;
        } else if (one_count > 0) {
            ans += 1;
        }

        return ans;
    }
};
// end_submission
