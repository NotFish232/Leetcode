#include <algorithm>
#include <limits>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>> &intervals) {
        sort(intervals.begin(), intervals.end(), [](const vector<int> &a, const vector<int> &b) {
            return a[1] == b[1] ? a[0] < b[0] : a[1] < b[1];
        });

        int end = numeric_limits<int>::min();
        int cnt = 0;

        for (const vector<int> &in : intervals) {
            if (in[0] < end) {
                ++cnt;
            } else {
                end = in[1];
            }
        }

        return cnt;
    }
};
// end_submission
