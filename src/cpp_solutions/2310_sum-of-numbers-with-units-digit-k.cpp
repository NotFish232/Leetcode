#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int minimumNumbers(int num, int k) {
        if (num == 0) {
            return 0;
        }

        if (k == 0) {
            return num % 10 == 0 ? 1 : -1;
        }

        int cnt = 0;

        while (num >= 0) {
            if (cnt != 0 && num % 10 == 0) {
                return cnt;
            }

            num -= k;
            cnt++;
        }

        return -1;
    }
};
// end_submission
