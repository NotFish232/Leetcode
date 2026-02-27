#include <bits/stdc++.h>
#include <climits>
#include <functional>
#include <queue>
#include <set>

using namespace std;

// start_submission
class Solution {
public:
    int maximumSum(vector<int> &arr) {
        int n = arr.size();

        vector<vector<int>> dp(2, vector<int>(n + 1, INT_MIN / 2));

        int ans = INT_MIN;

        for (int i = 0; i < n; ++i) {
            dp[0][i + 1] = max(dp[0][i] + arr[i], arr[i]);

            if (i > 0) {
                dp[1][i + 1] = max(dp[1][i] + arr[i], dp[0][i - 1] + arr[i]);
            }

            ans = max(ans, dp[0][i + 1]);
            ans = max(ans, dp[1][i + 1]);
        }

        return ans;
    }
};
// end_submission
