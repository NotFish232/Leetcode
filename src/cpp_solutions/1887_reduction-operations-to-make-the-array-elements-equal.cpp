#include <bits/stdc++.h>

using namespace std;

 // start_submission
class Solution {
public:
    int reductionOperations(vector<int>& nums) {
        unordered_map<int, int> m;
        for (const int &x: nums) {
            m[x]++;
        }


        vector<pair<int, int>> v(m.begin(), m.end());
        sort(v.rbegin(), v.rend());

        int ans = 0;
        for (int i = 0; i < v.size(); ++i) {
            ans += (v.size() - i - 1) * v[i].second;
        }

        return ans;
    }
};
 // end_submission
