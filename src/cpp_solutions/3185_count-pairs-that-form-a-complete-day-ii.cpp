#include <unordered_map>
#include <vector>

using namespace std;

// start_submission

class Solution {
public:
    long long countCompleteDayPairs(vector<int> &hours) {
        unordered_map<int, int> vals;

        long long ans = 0;

        for (const int &h : hours) {
            ans += vals[(24 - h % 24) % 24];

            ++vals[h % 24];
        }

        return ans;
    }
};
// end_submission
