#include <algorithm>
#include <string>

using namespace std;

// start_submission
class Solution {
public:
    int minLengthAfterRemovals(string s) {
        return s.size() - 2 * min(count(s.begin(), s.end(), 'a'), count(s.begin(), s.end(), 'b'));
    }
};
// end_submission
