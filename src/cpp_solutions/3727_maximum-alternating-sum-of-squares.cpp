#include <algorithm>
#include <bits/stdc++.h>
#include <cstdlib>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    long long maxAlternatingSum(vector<int> &nums) {
        int n = nums.size();

        for (int &x : nums) {
            x = abs(x);
        }

        sort(nums.rbegin(), nums.rend());

        long long res = 0;

        int s = (n + 1) / 2;

        for (int i = 0; i < n; ++i) {
            res += nums[i] * nums[i] * (i < s ? 1 : -1);
        }

        return res;
    }
};
// end_submission
