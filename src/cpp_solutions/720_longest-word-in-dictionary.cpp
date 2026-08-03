#include <bits/stdc++.h>
#include <set>
#include <string>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    string longestWord(vector<string>& words) {
        set<string> s{words.begin(), words.end()};

        string ans = "";

        for (const string &w: s) {
            bool good = true;
            for (int i = 1; i < w.size(); ++i) {
                if (s.find(w.substr(0, i)) == s.end()) {
                    good = false;
                    break;
                }
            }

            if (good) {
                if (w.size() > ans.size()) {
                    ans = w;
                }
            }
        }

        return ans;
    }
};
 // end_submission
