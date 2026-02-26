#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    typedef long long LL;

    long long countBadPairs(vector<int> &nums) {
        int n = nums.size();

        unordered_map<int, int> m;

        for (int i = 0; i < n; ++i) {
            m[nums[i] - i] += 1;
        }

        LL sum = 0;

        for (const auto &p : m) {
            sum += p.second;
        }

        LL ans = 0;

        for (const auto &p : m) {
            ans += p.second * (sum - p.second);
        }

        return ans / 2;
    }
};
// end_submission
