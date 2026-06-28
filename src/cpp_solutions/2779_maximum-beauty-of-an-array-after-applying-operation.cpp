#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int maximumBeauty(vector<int> &nums, int k) {
        unordered_map<int, pair<int, int>> m;
        for (const int &x : nums) {
            ++m[x - k].first;
            ++m[x + k].second;
        }

        vector<pair<int, pair<int, int>>> v(m.begin(), m.end());
        sort(v.begin(), v.end());

        int ans = 0;
        int count = 0;

        for (const auto &[_, p] : v) {
            count += p.first;

            ans = max(ans, count);

            count -= p.second;
        }

        return ans;
    }
};
// end_submission
