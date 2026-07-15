#include <vector>
#include <string>

using namespace std;

// start_submission

class Solution {
public:
    vector<int> build_z(string word) {
        int n = word.size();

        vector<int> z(n, 0);

        int l = 0, r = 0;

        for (int i = 1; i < n; ++i) {
            if (i <= r) {
                z[i] = min(z[i - l], r - i + 1);
            }

            while (i + z[i] < n && word[i + z[i]] == word[z[i]]) {
                ++z[i];
            }

            if (i + z[i] - 1 > r) {
                l = i;
                r = i + z[i] - 1;
            }
        }

        return z;
    }

    int minimumTimeToInitialState(string word, int k) {
        int n = word.size();

        vector<int> z = build_z(word);

        int cnt = 0;

        for (int i = 0; i < n; i += k) {
            if (z[i] == n - i) {
                return cnt;
            }

            ++cnt;
        }

        return cnt;
    }
};
// end_submission
