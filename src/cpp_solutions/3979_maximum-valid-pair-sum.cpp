#include <algorithm>
#include <climits>
#include <vector>

using namespace std;

// start_submission

class Solution {
public:
    int maxValidPairSum(vector<int> &nums, int k) {
        int n = nums.size();

        vector<int> p(n, 0);
        p[0] = nums[0];

        for (int i = 1; i < n; ++i) {
            p[i] = max(p[i - 1], nums[i]);
        }

        int ans = INT_MIN;

        for (int i = k; i < n; ++i) {
            ans = max(ans, p[i - k] + nums[i]);
        }

        return ans;
    }
};

// end_submission
