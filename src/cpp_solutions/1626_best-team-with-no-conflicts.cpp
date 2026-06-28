#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int bestTeamScore(vector<int> &scores, vector<int> &ages) {
        int n = scores.size();

        vector<pair<int, int>> v(n);
        for (int i = 0; i < n; ++i) {
            v[i] = {scores[i], ages[i]};
        }

        sort(v.begin(), v.end(), [&](const auto &a, const auto &b) {
            return a.first == b.first ? a.second < b.second : a.first < b.first;
        });

        vector<int> dp(n);
        for (int i = 0; i < n; ++i) {
            dp[i] = v[i].first;
        }

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < i; ++j) {
                if (v[i].second >= v[j].second) {
                    dp[i] = max(dp[i], dp[j] + v[i].first);
                }
            }
        }

        return *max_element(dp.begin(), dp.end());
    }
};
// end_submission
