#include <algorithm>
#include <limits>
#include <map>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int minimumCardPickup(vector<int> &cards) {
        int ans = numeric_limits<int>::max();
        map<int, int> m;
        for (int i = 0; i < cards.size(); ++i) {
            auto it = m.find(cards[i]);
            if (it != m.end()) {
                ans = min(ans, i - it->second + 1);
            }
            m[cards[i]] = i;
        }

        return ans == numeric_limits<int>::max() ? -1 : ans;
    }
};
// end_submission
