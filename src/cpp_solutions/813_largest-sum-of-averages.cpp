#include <algorithm>
#include <limits>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    double largestSumOfAverages(vector<int> &nums, int k) {
        int n = nums.size();

        vector<vector<double>> dp(n + 1, vector<double>(k, numeric_limits<double>::min() / 10));
        dp[0][0] = 0;

        for (int i = 0; i < n; ++i) {
            double cur = 0;

            for (int j = i; j >= 0; --j) {
                cur += nums[j];

                int offset = j != 0;

                for (int v = 0; v + offset < k; ++v) {
                    dp[i + 1][v + offset] = max(dp[i + 1][v + offset], dp[j][v] + cur / (i - j + 1));
                }
            }
        }

        return *max_element(dp[n].begin(), dp[n].end());
    }
};
// end_submission
