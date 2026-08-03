#include <algorithm>
#include <map>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int findMaxLength(vector<int> &nums) {
        int n = nums.size();

        for (int &x : nums) {
            if (x == 0) {
                x = -1;
            }
        }

        vector<int> psums(n + 1, 0);
        for (int i = 0; i < n; ++i) {
            psums[i + 1] = psums[i] + nums[i];
        }

        map<int, int> prev;
        int ans = 0;
        for (int i = 0; i <= n; ++i) {
            auto it = prev.find(psums[i]);

            if (it == prev.end()) {
                prev[psums[i]] = i;
            } else {
                ans = max(ans, i - it->second);
            }
        }

        return ans;
    }
};
// end_submission
