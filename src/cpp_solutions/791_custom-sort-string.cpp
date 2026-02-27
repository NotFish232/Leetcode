#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    string customSortString(string order, string s) {
        unordered_map<char, int> m;
        for (int i = 0; i < order.size(); ++i) {
            m[order[i]] = i;
        }

        sort(s.begin(), s.end(), [&](const char &a, const char &b) {
            int a_ord = m[a];
            int b_ord = m[b];

            return a_ord < b_ord;
        });

        return s;
    }
};
// end_submission
