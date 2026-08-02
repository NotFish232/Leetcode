#include <algorithm>
#include <deque>
#include <limits>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int shortestSubarray(vector<int> &nums, int k) {
        int n = nums.size();

        vector<long long> psum(n + 1, 0);

        for (int i = 0; i < nums.size(); ++i) {
            psum[i + 1] = psum[i] + nums[i];
        }

        int ans = numeric_limits<int>::max();

        deque<int> q;

        for (int i = 0; i <= n; ++i) {
            while (!q.empty() && psum[q.back()] >= psum[i]) {
                q.pop_back();
            }
            while (!q.empty() && psum[i] - psum[q.front()] >= k) {
                ans = min(ans, i - q.front());
                q.pop_front();
            }
            q.push_back(i);
        }

        return ans != numeric_limits<int>::max() ? ans : -1;
    }
};
// end_submission
