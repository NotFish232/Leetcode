#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int maxOperations(vector<int> &nums, int k) {
        sort(nums.begin(), nums.end());

        int cnt = 0;

        int l = 0;
        int r = nums.size() - 1;

        while (l < r) {
            if (nums[l] + nums[r] == k) {
                cnt++;
                l++;
                r--;
            } else if (nums[l] + nums[r] < k) {
                l++;
            } else {
                r--;
            }
        }

        return cnt;
    }
};
// end_submission
