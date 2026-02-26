#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    typedef long long LL;

    long long findMaximumScore(vector<int> &nums) {
        int cur_max = nums[0];

        LL ans{0};

        for (int i{1}; i < nums.size(); ++i) {
            ans += cur_max;

            cur_max = max(cur_max, nums[i]);
        }

        return ans;
    }
};
// end_submission
