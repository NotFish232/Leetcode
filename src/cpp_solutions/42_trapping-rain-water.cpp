#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int trap(vector<int> &height) {
        int n = height.size();

        vector<int> l_max(n + 1, 0);
        vector<int> r_max(n + 1, 0);

        for (int i = 0; i < height.size(); ++i) {
            l_max[i + 1] = max(l_max[i], height[i]);
        }

        for (int i = height.size() - 1; i >= 0; --i) {
            r_max[i] = max(r_max[i + 1], height[i]);
        }

        int cnt = 0;

        for (int i = 1; i < n - 1; ++i) {
            cnt += min(l_max[i + 1], r_max[i]) - height[i];
        }

        return cnt;
    }
};
// end_submission
