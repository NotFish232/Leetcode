#include <algorithm>
#include <limits>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int maxProfit(vector<int> &prices, int fee) {
        int n = prices.size();
        vector<vector<long long>> dp(n + 1);
        dp[0] = {0, numeric_limits<long long>::min() / 10};

        for (int i = 0; i < n; ++i) {
            dp[i + 1] = dp[i];

            dp[i + 1][0] = max(dp[i + 1][0], dp[i][1] + prices[i] - fee);
            dp[i + 1][1] = max(dp[i + 1][1], dp[i][0] - prices[i]);
        }

        return max(dp[n][0], dp[n][1]);
    }
};
// end_submission
