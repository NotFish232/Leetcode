#include <numeric>
#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    bool doesValidArrayExist(vector<int>& derived) {
        return accumulate(derived.begin(), derived.end(), 0, [](int acc, int el) {
            return acc ^ el;
        }) == 0;
    }
};
 // end_submission
