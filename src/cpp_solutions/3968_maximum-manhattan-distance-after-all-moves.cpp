#include <bits/stdc++.h>
#include <cstdlib>
#include <string>

using namespace std;

 // start_submission
class Solution {
public:
    int maxDistance(string moves) {
        int x = 0, y = 0;
        int cnt = 0;

        for (const char &ch: moves) {
            if (ch == 'L') {
                --x;
            } else if (ch == 'R') {
                ++x;
            } else if (ch == 'U') {
                --y;
            } else if (ch == 'D') {
                ++y;
            } else {
                ++cnt;
            }
        }

        return abs(x) + abs(y) + cnt;
    }
};
 // end_submission
