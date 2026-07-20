#include <algorithm>
#include <bits/stdc++.h>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int peakIndexInMountainArray(vector<int> &arr) {
        int l = 1, r = arr.size() - 2;

        while (l <= r) {
            int m = l + (r - l) / 2;

            if (arr[m] > max(arr[m - 1], arr[m + 1])) {
                return m;
            }

            if (arr[m] > arr[m - 1]) {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        return -1;
    }
};
// end_submission
