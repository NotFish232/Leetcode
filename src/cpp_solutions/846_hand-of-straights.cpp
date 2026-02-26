#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    bool isNStraightHand(vector<int> &hand, int groupSize) {
        int n = hand.size();

        if (n % groupSize != 0) {
            return false;
        }

        multiset<int> ms;

        for (const auto &x : hand) {
            ms.insert(x);
        }

        int t = n / groupSize;

        while (t--) {
            int start = *ms.begin();
            ms.erase(ms.begin());

            for (int i = 0; i + 1 < groupSize; ++i) {
                start += 1;

                auto it = ms.find(start);

                if (it == ms.end()) {
                    return false;
                }

                ms.erase(it);
            }
        }

        return true;
    }
};
// end_submission
