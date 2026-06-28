#include <bits/stdc++.h>

using namespace std;

 // start_submission
class Solution {
public:
    int maximumElementAfterDecrementingAndRearranging(vector<int>& arr) {
        sort(arr.begin(), arr.end());

        int ans = 0;
        
        for (const int &x: arr) {
            ans = min(ans + 1, x);
        }

        return ans;
    }
};
 // end_submission
