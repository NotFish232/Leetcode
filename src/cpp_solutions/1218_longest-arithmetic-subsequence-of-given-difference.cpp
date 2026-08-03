#include <algorithm>
#include <map>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    int longestSubsequence(vector<int>& arr, int difference) {
        int ans = 1;

        map<int, int> m;
        for (const int &x: arr) {
            if (m.find(x - difference) != m.end()) {
                m[x] = m[x - difference] + 1;
                ans = max(ans, m[x]);
            } else {
                m[x] = 1;
            }
        }

        return ans;
    }
};
 // end_submission
