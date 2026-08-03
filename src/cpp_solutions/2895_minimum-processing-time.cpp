#include <algorithm>
#include <limits>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int minProcessingTime(vector<int> &processorTime, vector<int> &tasks) {
        sort(processorTime.rbegin(), processorTime.rend());
        sort(tasks.begin(), tasks.end());

        int ans = numeric_limits<int>::min();

        for (int i = 0; i < processorTime.size(); ++i) {
            ans = max(ans, processorTime[i] + *max_element(tasks.begin() + (i * 4), tasks.begin() + ((i + 1) * 4)));
        }

        return ans;
    }
};
// end_submission
