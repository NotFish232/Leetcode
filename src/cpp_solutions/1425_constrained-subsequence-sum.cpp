#include <algorithm>
#include <deque>
#include <limits>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int constrainedSubsetSum(vector<int> &nums, int k) {
        int n = nums.size();

        int ans = numeric_limits<int>::min();

        deque<pair<int, int>> q;

        // big to small

        for (int i = 0; i < n; ++i) {
            if (q.size() > 0 && q.front().first + k < i) {
                q.pop_front();
            }

            pair<int, int> el = {i, nums[i]};
            if (q.size() > 0 && q.front().second > 0) {
                el.second += q.front().second;
            }

            while (q.size() > 0 && el.second >= q.back().second) {
                q.pop_back();
            }
            q.push_back(el);
            ans = max(ans, el.second);
        }

        return ans;
    }
};
// end_submission
