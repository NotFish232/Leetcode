#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    vector<bool> isArraySpecial(vector<int> &nums, vector<vector<int>> &queries) {
        int n = nums.size();

        vector<int> p_sum(n, 0);
        for (int i = 1; i < n; ++i) {
            p_sum[i] = p_sum[i - 1] + (nums[i - 1] % 2 != nums[i] % 2);
        }

        vector<bool> res;

        for (const vector<int> &q : queries) {
            int l = q[0];
            int r = q[1];

            res.push_back(p_sum[r] - p_sum[l] == r - l);
        }

        return res;
    }
};
// end_submission
