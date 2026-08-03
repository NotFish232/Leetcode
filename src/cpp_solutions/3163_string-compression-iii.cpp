#include <algorithm>
#include <bits/stdc++.h>
#include <string>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    string compressedString(string word) {
        vector<char> out;
        char prev = '#';
        int cnt = 0;
        for (const char &ch: word) {
            if (prev != ch) {
                while (cnt > 0) {
                    out.push_back('0' + min(cnt, 9));
                    out.push_back(prev);
                    cnt = max(cnt - 9, 0);
                }
                
                cnt = 1;
                prev = ch;
            } else {
                ++cnt;
            }
        }

        while (cnt > 0) {
            out.push_back('0' + min(cnt, 9));
            out.push_back(prev);
            cnt = max(cnt - 9, 0);
        }

        return {out.begin(), out.end()};
    }
};
 // end_submission
