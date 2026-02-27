#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int maxSubstrings(string word) {
        int n = word.size();

        unordered_map<char, int> m;

        int cnt = 0;

        for (int i = 0; i < word.size(); ++i) {
            if (m.find(word[i]) != m.end()) {
                if (i - m[word[i]] >= 3) {
                    cnt++;
                    m.clear();
                }
            } else {
                m[word[i]] = i;
            }
        }

        return cnt;
    }
};
// end_submission
