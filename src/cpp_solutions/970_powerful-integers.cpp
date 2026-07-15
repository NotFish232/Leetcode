#include <set>
#include <vector>

using namespace std;

// start_submission

class Solution {
public:
    long long ipow(long long base, int exp) {
        long long res = 1;

        while (exp > 0) {
            if (exp % 2 == 1) {
                res *= base;
            }

            base *= base;
            exp /= 2;
        }

        return res;
    }

    vector<int> powerfulIntegers(int x, int y, int bound) {
        set<int> vals;

        for (int i = 0; ipow(x, i) <= bound && (x != 1 || i == 0); ++i) {
            for (int j = 0; ipow(y, j) + ipow(x, i) <= bound && (y != 1 || j == 0); ++j) {
                vals.insert(ipow(x, i) + ipow(y, j));
            }
        }

        return {vals.begin(), vals.end()};
    }
};

// end_submission
