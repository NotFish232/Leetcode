#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    bool hasValidPath(vector<vector<char>> &grid) {
        int n = grid.size(), m = grid[0].size();

        vector<vector<vector<bool>>> dp(n,
                                        vector<vector<bool>>(m, vector<bool>(n + m + 1, false)));

        if (grid[0][0] == '(') {
            dp[0][0][1] = true;
        }

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                int d = grid[i][j] == '(' ? 1 : -1;

                for (int k = d == -1; k + (d == 1) <= n + m; ++k) {
                    if (i > 0 && dp[i - 1][j][k]) {
                        dp[i][j][k + d] = true;
                    }

                    if (j > 0 && dp[i][j - 1][k]) {
                        dp[i][j][k + d] = true;
                    }
                }
            }
        }

        return dp[n - 1][m - 1][0];
    }
};
// end_submission
