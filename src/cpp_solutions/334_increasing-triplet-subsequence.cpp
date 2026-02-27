#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool increasingTriplet(vector<int> &nums) {
        int cur_min_1 = INT_MAX;
        int cur_min_2 = INT_MAX;
        int cur_min_3 = INT_MAX;

        for (const int &x : nums) {
            if (x > cur_min_2) {
                return true;
            }
            if (x > cur_min_1) {
                cur_min_2 = min(cur_min_2, x);
            }

            cur_min_1 = min(cur_min_1, x);
        }

        return false;
    }
};
// end_submission
