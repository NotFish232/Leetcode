#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    vector<int> sortByBits(vector<int> &arr) {
        sort(arr.begin(), arr.end(), [&](const auto &a, const auto &b) {
            int a_count = __builtin_popcount(a);
            int b_count = __builtin_popcount(b);

            return a_count == b_count ? a <= b : a_count <= b_count;
        });

        return arr;
    }
};
// end_submission
