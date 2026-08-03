#include <bits/stdc++.h>
#include <numeric>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int numberOfGoodSubarraySplits(vector<int> &nums) {
        int n = nums.size();

        if (accumulate(nums.begin(), nums.end(), 0) == 0) {
            return 0;
        }

        vector<int> gaps;

        int i = 0;
        while (i < n && nums[i] == 0) {
            ++i;
        }
        ++i;

        for (; i < n; ++i) {
            int cnt = 0;
            while (i < n && nums[i] == 0) {
                ++cnt;
                ++i;
            }

            if (i != n) {
                gaps.push_back(cnt);
            }
        }

        return accumulate(gaps.begin(), gaps.end(), 1LL, [](const long long &acc, const int &el) {
            return (acc * (long long)(el + 1)) % (1'000'000'007);
        });
    }
};
// end_submission
