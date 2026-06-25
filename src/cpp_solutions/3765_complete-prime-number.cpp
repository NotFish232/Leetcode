#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool isPrime(int x) {
        if (x < 2) {
            return false;
        }

        for (int f = 2; f * f <= x; ++f) {
            if (x % f == 0) {
                return false;
            }
        }

        return true;
    }

    bool completePrime(int num) {
        string s = to_string(num);

        for (int i = 0; i < s.size(); ++i) {
            if (!isPrime(stoi(s.substr(i))) || !isPrime(stoi(s.substr(0, s.size() - i)))) {
                return false;
            }
        }

        return true;
    }
};
// end_submission
