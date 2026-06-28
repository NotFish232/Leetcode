#include <bits/stdc++.h>

using namespace std;

 // start_submission
class Solution {
public:
    long long maxEnergyBoost(vector<int>& energyDrinkA, vector<int>& energyDrinkB) {
        int n = energyDrinkA.size();
        
        vector<vector<long long>> dp(n + 1, vector<long long>(2, 0));

        for (int i = 0; i < n; ++i) {
            dp[i + 1][0] = dp[i][0] + energyDrinkA[i];
            dp[i + 1][1] = dp[i][1] + energyDrinkB[i];

            if (i > 0) {
                dp[i + 1][0] = max(dp[i + 1][0], dp[i - 1][1] + energyDrinkA[i]);
                dp[i + 1][1] = max(dp[i + 1][1], dp[i - 1][0] + energyDrinkB[i]);
            }
        }

        return max(dp[n][0], dp[n][1]);
    }
};
 // end_submission
