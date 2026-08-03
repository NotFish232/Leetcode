#include <bits/stdc++.h>
#include <string>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int maxDiff(int num) {
        string s = to_string(num);
        vector<char> a{s.begin(), s.end()}, b{s.begin(), s.end()};

        for (int i = 0; i < a.size(); ++i) {
            char ch = a[i];

            if (ch != '9') {
                for (int j = 0; j < a.size(); ++j) {
                    if (a[j] == ch) {
                        a[j] = '9';
                    }
                }
                break;
            }
        }

        for (int i = 0; i < b.size(); ++i) {
            char ch = b[i];

            if ((i == 0 && ch != '1') || (i != 0 && ch != '0' && b[0] != ch)) {
                for (int j = 0; j < b.size(); ++j) {
                    if (b[j] == ch) {
                        b[j] = i == 0 ? '1' : '0';
                    }
                }
                break;
            }
        }

        string as{a.begin(), a.end()}, bs{b.begin(), b.end()};

        return stoi(as) - stoi(bs);
    }
};
// end_submission
