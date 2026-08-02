#include <array>
#include <limits>
#include <string>

using namespace std;

// start_submission
class Solution {
public:
    string minWindow(string s, string t) {
        auto ctoi = [](const char &ch) {
            return ch >= 'a' ? (ch - 'a') : (ch - 'A' + 26);
        };

        array<int, 52> freq{};
        for (const char &ch : t) {
            ++freq[ctoi(ch)];
        }

        array<int, 52> cur_freq{};

        int need = t.size();

        int l = 0;

        int len = numeric_limits<int>::max();
        int idx = -1;

        for (int r = 0; r < s.size(); ++r) {
            if (++cur_freq[ctoi(s[r])] <= freq[ctoi(s[r])]) {
                --need;
            }

            while (need == 0 && cur_freq[ctoi(s[l])] - 1 >= freq[ctoi(s[l])]) {
                --cur_freq[ctoi(s[l])];
                ++l;
            }

            if (need == 0 && r - l + 1 < len) {
                len = r - l + 1;
                idx = l;
            }
        }

        return idx == -1 ? "" : s.substr(idx, len);
    }
};
// end_submission
