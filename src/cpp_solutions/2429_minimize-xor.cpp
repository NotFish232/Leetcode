#include <algorithm>
#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
private:
    int bits(int x) {
        int cnt = 0;
        while (x) {
            x >>= 1;
            ++cnt;
        }

        return cnt;
    }

public:
    int minimizeXor(int num1, int num2) {
        int n = max(bits(num1), bits(num2));
        int c1 = __builtin_popcount(num2);
        int c0 = n - c1;

        int x = 0;

        while (n--) {
            x <<= 1;

            if (((num1 >> n) & 1) == 1) {
                if (c1 > 0) {
                    ++x;
                    --c1;
                } else {
                    --c0;
                }
            } else {
                if (c0 > 0) {
                    --c0;
                } else {
                    ++x;
                    --c1;
                }
            }
        }

        return x;
    }
};
// end_submission
