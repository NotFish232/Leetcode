#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    vector<int> mostCompetitive(vector<int> &nums, int k) {
        vector<int> stk;

        for (int i = 0; i < nums.size(); ++i) {
            while (stk.size() > 0 && nums[i] < stk.back() && stk.size() + nums.size() - i - 1 >= k) {
                stk.pop_back();
            }

            stk.push_back(nums[i]);
        }

        while (stk.size() > k) {
            stk.pop_back();
        }

        return stk;
    }
};
// end_submission
