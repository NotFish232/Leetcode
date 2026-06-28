#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    long long continuousSubarrays(vector<int> &nums) {
        int n = nums.size();

        unordered_map<int, int> m;

        long long ans = 0;

        int l = 0;

        for (int r = 0; r < n; ++r) {
            while (l <= r) {
                int cnt = 0;
                for (int d = -2; d <= 2; ++d) {
                    cnt += m[nums[r] + d];
                }

                if (r - l == cnt) {
                    break;
                }

                --m[nums[l]];
                ++l;
            }

            ++m[nums[r]];

            ans += r - l + 1;
        }

        return ans;
    }
};
// end_submission
