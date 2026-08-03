#include <algorithm>
#include <bits/stdc++.h>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    long long maxArrayValue(vector<int> &nums) {
        int n = nums.size();

        vector<long long> v{nums.begin(), nums.end()};

        for (int i = n - 1; i > 0; --i) {
            if (v[i] >= v[i - 1]) {
                v[i - 1] += v[i];
            }
        }

        return *max_element(v.begin(), v.end());
    }
};
// end_submission
