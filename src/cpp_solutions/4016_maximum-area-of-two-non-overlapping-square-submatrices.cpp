#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int maxArea(vector<vector<int>> &mat) {
        int n = mat.size(), m = mat[0].size();

        vector<vector<int>> pre(n + 1, vector<int>(m + 1, 0));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                pre[i + 1][j + 1] = pre[i + 1][j] + pre[i][j + 1] - pre[i][j] + mat[i][j];
            }
        }

        int l = 0, r = min(n, m);
        int ans = 0;

        while (l <= r) {
            int k = l + (r - l) / 2;

            vector<pair<int, int>> squares;

            for (int i = k; i <= n; ++i) {
                for (int j = k; j <= m; ++j) {
                    int cnt = pre[i][j] - pre[i - k][j] - pre[i][j - k] + pre[i - k][j - k];

                    if (cnt == k * k) {
                        squares.push_back({i - k, j - k});
                    }
                }
            }

            bool good = false;

            vector<int> a, b;

            for (const auto &[x, y] : squares) {
                a.push_back(x);
                b.push_back(y);
            }

            if (squares.size() > 0) {
                if (*max_element(a.begin(), a.end()) >= *min_element(a.begin(), a.end()) + k) {
                    good = true;
                }
                if (*max_element(b.begin(), b.end()) >= *min_element(b.begin(), b.end()) + k) {
                    good = true;
                }
            }

            if (good) {
                ans = k;
                l = k + 1;
            } else {
                r = k - 1;
            }
        }

        return ans * ans;
    }
};
// end_submission
