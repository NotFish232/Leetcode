#include <queue>
#include <string>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    long long maximumScore(vector<int>& nums, string s) {
        priority_queue<int> q;

        long long ans = 0;

        for (int i = 0; i < nums.size(); ++i) {
            q.push(nums[i]);

            if (s[i] == '1') {
                ans += q.top();
                q.pop();
            }
        }

        return ans;
    }
};
 // end_submission
