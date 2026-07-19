#include <algorithm>
#include <limits>
#include <utility>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    long long maximumTotalCost(vector<int>& nums) {
        int n = nums.size();
        
        vector<pair<long long, long long>> dp(n + 1, {0, 0});
        dp[0].first = numeric_limits<int>::min();

        for (int i = 0; i < n; ++i) {
            dp[i + 1].first = max(dp[i].first + nums[i], dp[i].second + nums[i]);
            dp[i + 1].second = dp[i].first - nums[i];
        }

        return max(dp[n].first, dp[n].second);
    }
};
 // end_submission
