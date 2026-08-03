#include <algorithm>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    int minimizedMaximum(int n, vector<int>& quantities) {
        int l = 1, r = *max_element(quantities.begin(), quantities.end());

        int ans = -1;

        while (l <= r) {
            int m = l + (r - l) / 2;

            int cnt = 0;
            for (const int &q: quantities) {
                cnt += (q + m - 1) / m;
            }

            if (cnt <= n) {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        return ans;
    }
};
 // end_submission
