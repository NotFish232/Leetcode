#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int minTaps(int n, vector<int> &ranges) {
        vector<pair<int, int>> v;
        for (int i = 0; i < n + 1; ++i) {
            v.emplace_back(max(0, i - ranges[i]), min(i + ranges[i], n));
        }

        sort(v.begin(), v.end());

        int used_idx = 0, current_r = 0, possible_r = 0, cnt = 0;

        for (int i = 0; i <= n; ++i) {
            if (i > current_r || (i == current_r && i != n)) {
                for (; used_idx <= n && v[used_idx].first <= i; ++used_idx) {
                    possible_r = max(possible_r, v[used_idx].second);
                }
                
                if (possible_r <= i) {
                    return -1;
                }

                cnt++;
                current_r = possible_r;
                possible_r = 0;
            }
        }

        return cnt;
    }
};
// end_submission
