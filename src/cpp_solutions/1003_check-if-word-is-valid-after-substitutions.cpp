#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool isValid(string s) {
        vector<char> stk;

        for (const char &ch : s) {
            if (ch == 'a' || ch == 'b') {
                stk.push_back(ch);
            } else if (ch == 'c') {
                if (stk.size() < 2) {
                    return false;
                }
                char ch1 = stk.back();
                stk.pop_back();
                char ch2 = stk.back();
                stk.pop_back();

                if (!(ch1 == 'b' && ch2 == 'a')) {
                    return false;
                }
            }
        }

        return stk.size() == 0;
    }
};
// end_submission
