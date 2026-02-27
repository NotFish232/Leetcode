#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int minLength(vector<int> &nums, int k) {
        int n = nums.size();

        unordered_map<int, int> m;

        int cur_sum = 0;

        int r_ptr = 0;

        int ans = INT_MAX;

        for (int l_ptr = 0; l_ptr < n; ++l_ptr) {
            while (r_ptr < n && cur_sum < k) {
                m[nums[r_ptr]]++;
                if (m[nums[r_ptr]] == 1) {
                    cur_sum += nums[r_ptr];
                }
                r_ptr++;
            }

            if (cur_sum >= k) {
                ans = min(ans, r_ptr - l_ptr);
            }

            m[nums[l_ptr]]--;

            if (m[nums[l_ptr]] == 0) {
                m.erase(nums[l_ptr]);
                cur_sum -= nums[l_ptr];
            }
        }

        return ans == INT_MAX ? -1 : ans;
    }
};
// end_submission
