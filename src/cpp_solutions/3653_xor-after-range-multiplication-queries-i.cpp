#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    typedef long long LL;

    int xorAfterQueries(vector<int> &nums, vector<vector<int>> &queries) {
        LL mod = pow(10, 9) + 7;

        for (const vector<int> &q : queries) {
            int l = q[0];
            int r = q[1];
            int k = q[2];
            int v = q[3];

            for (int i = l; i <= r; i += k) {
                nums[i] = ((LL)nums[i] * (LL)v) % mod;
            }
        }

        int ans = 0;

        for (const int &x : nums) {
            ans ^= x;
        }

        return ans;
    }
};
// end_submission
