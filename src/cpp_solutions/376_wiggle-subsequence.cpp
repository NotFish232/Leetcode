#include <bits/stdc++.h>
#include <ostream>

using namespace std;

// start_submission
class Solution {
public:
    int wiggleMaxLength(vector<int> &nums) {
        int dp[nums.size()][2];
        dp[0][0] = 1;
        dp[0][1] = 1;
        int el[nums.size()][2];
        el[0][0] = nums[0];
        el[0][1] = nums[0];


        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] > el[i - 1][1]) {
                dp[i][0] = dp[i - 1][1] + 1;
                el[i][0] = nums[i];
            } else {
                dp[i][0] = dp[i - 1][0];
                el[i][0] = max(nums[i], el[i - 1][0]);
            }

            if (nums[i] < el[i - 1][0]) {
                dp[i][1] = dp[i - 1][0] + 1;
                el[i][1] = nums[i];
            } else {
                dp[i][1] = dp[i - 1][1];
                el[i][1] = min(nums[i], el[i - 1][1]);
            }
        }

        return max(dp[nums.size() - 1][0], dp[nums.size() - 1][1]);
    }
};
// end_submission
