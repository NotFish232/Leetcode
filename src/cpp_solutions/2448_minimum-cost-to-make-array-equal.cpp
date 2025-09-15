#include <bits/stdc++.h>

using namespace std;

// start_submission
typedef unsigned long long ULL;
typedef pair<ULL, ULL> PII;

class Solution {
public:
    long long minCost(vector<int> &nums, vector<int> &cost) {
        vector<PII> v;
        for (int i = 0; i < nums.size(); ++i) {
            v.emplace_back(nums[i], cost[i]);
        }
        sort(v.begin(), v.end(), [&](const PII &a, const PII &b) {
            return a.first < b.first;
        });

        ULL cur_val = 0;
        ULL cost_pre_sum = v[0].second;
        ULL cost_post_sum = 0;

        for (int i = 0; i < v.size(); ++i) {
            cur_val += (v[i].first - v[0].first) * v[i].second;
            cost_post_sum += v[i].second;
        }
        cost_post_sum -= v[0].second;

        ULL ans = cur_val;

        for (int i = 1; i < v.size(); ++i) {
            cur_val += cost_pre_sum * (v[i].first - v[i - 1].first);
            cur_val -= cost_post_sum * (v[i].first - v[i - 1].first);

            ans = min(ans, cur_val);

            cost_pre_sum += v[i].second;
            cost_post_sum -= v[i].second;

        }

        return ans;
    }
};
// end_submission
