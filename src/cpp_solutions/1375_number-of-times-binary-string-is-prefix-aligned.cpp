#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int numTimesAllBlue(vector<int> &flips) {
        int n = flips.size();

        vector<int> v(n);

        int cnt = 0;
        int ans = 0;

        for (int i = 0; i < n; ++i) {
            if (flips[i] - 1 > i) {
                v[flips[i] - 1] = 1;
            } else {
                ++cnt;
            }
            cnt += v[i];

            if (cnt == i + 1) {
                ++ans;
            }
        }

        return ans;
    }
};
// end_submission
