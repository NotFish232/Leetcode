#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int change(int amount, vector<int> &coins) {
        vector<unsigned long long> dp(amount + 1);
        dp[0] = 1;

        for (const int &coin : coins) {
            for (int i = 1; i <= amount; ++i) {
                if (i >= coin) {
                    dp[i] += dp[i - coin];
                }
            }
        }
        
        return dp[amount];
    }
};
// end_submission
