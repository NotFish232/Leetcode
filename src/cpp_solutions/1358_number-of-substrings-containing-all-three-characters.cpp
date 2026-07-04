#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int numberOfSubstrings(string s) {
        int n = s.size();

        unordered_map<char, int> m;

        int ans = 0;

        int l = 0;

        for (int r = 0; r < n; ++r) {
            ++m[s[r]];

            while (l < r && m.size() == 3 && m[s[l]] > 1) {
                --m[s[l]];
                ++l;
            }

            if (m.size() == 3) {
                ans += l + 1;
            }
        }

        return ans;
    }
};
// end_submission
