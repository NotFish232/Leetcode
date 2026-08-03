#include <numeric>
#include <string>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    string maximumXor(string s, string t) {
        int n = s.size();

        int cnt1 = accumulate(t.begin(), t.end(), 0, [](auto acc, auto el) {
            return acc + (el == '1');
        });

        int cnt0 = n - cnt1;

        vector<char> v;

        for (const auto &ch : s) {
            if (ch == '1' && cnt0 > 0) {
                --cnt0;
                v.push_back('1');
            } else if (ch == '0' && cnt1 > 0) {
                --cnt1;
                v.push_back('1');
            } else {
                v.push_back('0');
            }
        }

        return {v.begin(), v.end()};
    }
};
// end_submission
