#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    vector<int> twoSum(vector<int> &nums, int target) {
        map<int, int> m;

        for (int i = 0; i < nums.size(); ++i) {
            int x = nums[i];

            int comp = target - x;
            if (m.find(comp) != m.end()) {
                return {i, m[comp]};
            }

            m[x] = i;
        }

        __builtin_unreachable();
    }
};
// end_submission
