#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    string largestWordCount(vector<string> &messages, vector<string> &senders) {
        unordered_map<string, int> m;

        for (int i = 0; i < messages.size(); ++i) {
            m[senders[i]] += count(messages[i].begin(), messages[i].end(), ' ') + 1;
        }

        return max_element(m.begin(), m.end(), [&](const auto &a, const auto &b) {
                   return a.second == b.second ? a.first < b.first : a.second < b.second;
               })
            ->first;
    }
};
// end_submission
