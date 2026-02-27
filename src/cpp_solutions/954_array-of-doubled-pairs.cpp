#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool canReorderDoubled(vector<int> &arr) {
        auto cmp = [&](const auto &a, const auto &b) {
            return abs(a) == abs(b) ? a < b : abs(a) < abs(b);
        };

        multiset<int, decltype(cmp)> ms(cmp);

        for (const int &x : arr) {
            ms.insert(x);
        }

        while (ms.size() > 0) {
            int first = *ms.begin();
            ms.erase(ms.begin());

            auto it = ms.find(2 * first);

            if (it == ms.end()) {
                return false;
            }

            ms.erase(it);
        }

        return true;
    }
};
// end_submission
