#include <algorithm>
#include <bits/stdc++.h>
#include <limits>
#include <map>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int minOperations(vector<int> &nums, int x) {
        int n = nums.size();

        int sum = 0;
        map<int, vector<int>> post;
        post[sum].push_back(0);
        for (int i = n - 1; i >= 0; --i) {
            sum += nums[i];
            post[sum].push_back(n - i);
        }

        int cur = 0;

        int ans = numeric_limits<int>::max();

        for (int i = 0; i <= n; ++i) {
            auto it = post.find(x - cur);

            if (it != post.end()) {
                ans = min(ans, i + it->second.front());
            }

            post[sum].pop_back();
            if (post[sum].size() == 0) {
                post.erase(sum);
            }

            if (i < n) {
                cur += nums[i];
                sum -= nums[i];
            }
        }

        return ans == numeric_limits<int>::max() ? -1 : ans;
    }
};
// end_submission
