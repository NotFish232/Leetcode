#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int maximumXOR(vector<int> &nums) {
        int ans = 0;

        for (const int &x : nums) {
            ans |= x;
        }

        return ans;
    }
};
// end_submission
