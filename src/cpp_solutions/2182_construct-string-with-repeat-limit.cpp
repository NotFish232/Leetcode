#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    string repeatLimitedString(string s, int repeatLimit) {
        auto cmp = [&](const char &a, const char &b) {
            return a > b;
        };

        map<char, int, decltype(cmp)> m(cmp);

        for (const char &ch : s) {
            m[ch]++;
        }

        string res;

        int repeats = 0;

        while (m.size() > 0) {
            const auto it = m.begin();

            if (repeats < repeatLimit) {
                res += it->first;
                repeats++;

                it->second--;

                if (it->second == 0) {
                    m.erase(it);
                    repeats = 0;
                }
            } else {
                if (m.size() == 1) {
                    break;
                } else {
                    int k = it->first;
                    int v = it->second;

                    m.erase(it);

                    auto new_it = m.begin();

                    res += new_it->first;

                    new_it->second--;
                    if (new_it->second == 0) {
                        m.erase(new_it);
                    }

                    m[k] = v;
                    repeats = 0;
                }
            }
        }

        return res;
    }
};
// end_submission
