#include <bits/stdc++.h>
#include <unordered_map>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    int numberWays(vector<vector<int>>& hats) {
        const int mod = 1'000'000'007;
        const int num_hats = 40;
        
        int n = hats.size();

        vector<vector<int>> hat_to_ppl(num_hats + 1);
        for (int p = 0; p < n; ++p) {
            for (const int &h: hats[p]) {
                hat_to_ppl[h].push_back(p);
            }
        }
        
        unordered_map<int, long long> dp = {};
        dp[0] = 1;

        for (int h = 1; h <= num_hats; ++h) {
            unordered_map<int, long long> new_dp {dp.begin(), dp.end()};
            
            for (const int &p: hat_to_ppl[h]) {
                for (auto [k, v]: dp) {
                    if (((k >> p) & 1) == 0) {
                        new_dp[k | (1 << p)] += v;
                        new_dp[k | (1 << p)] %= mod;
                    }
                }
            }

            dp = new_dp;
        }

        return dp[(1 << n) - 1];
        
    }
};
 // end_submission
