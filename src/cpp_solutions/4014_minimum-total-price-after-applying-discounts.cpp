#include <algorithm>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    double minPrice(vector<int> &prices, vector<int> &discounts) {
        sort(prices.rbegin(), prices.rend());
        sort(discounts.rbegin(), discounts.rend());

        double ans = 0;

        for (int i = 0; i < prices.size(); ++i) {
            ans += prices[i] * (100.0 - (i < discounts.size() ? discounts[i] : 0)) / 100.0;
        }

        return ans;
    }
};
// end_submission
