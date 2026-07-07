#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool validateStackSequences(vector<int> &pushed, vector<int> &popped) {
        int n = pushed.size();

        vector<int> stk;

        int i = 0, j = 0;

        while (i < n || j < n) {
            if (stk.size() > 0 && j < n && stk.back() == popped[j]) {
                stk.pop_back();
                ++j;
            } else {
                if (i == n) {
                    break;
                }
                stk.push_back(pushed[i++]);
            }
        }

        return stk.size() == 0;
    }
};
// end_submission
