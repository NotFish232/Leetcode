#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
private:
    pair<int, int> smallest_two(vector<int> &v) {
        int s1 = -1, s2 = -1;

        for (int i = 0; i < v.size(); ++i) {
            if (s1 == -1 || v[i] < v[s1]) {
                s2 = s1;
                s1 = i;
            } else if (s2 == -1 || v[i] < v[s2]) {
                s2 = i;
            }
        }

        return {s1, s2};
    }

public:
    int minFallingPathSum(vector<vector<int>> &grid) {
        int n = grid.size();

        if (n == 1) {
            return grid[0][0];
        }

        auto [s1, s2] = smallest_two(grid[0]);
        int v1 = grid[0][s1], v2 = grid[0][s2];

        for (int i = 1; i < n; ++i) {
            int i1 = -1, i2 = -1;

            for (int j = 0; j < n; ++j) {
                if (i1 == -1 || grid[i][j] + (j != s1 ? v1 : v2) < grid[i][i1] + (i1 != s1 ? v1 : v2)) {
                    i2 = i1;
                    i1 = j;
                } else if (i2 == -1 || grid[i][j] + (j != s1 ? v1 : v2) < grid[i][i2] + (i2 != s1 ? v1 : v2)) {
                    i2 = j;
                }
            }

            int n_v1 = grid[i][i1] + (i1 != s1 ? v1 : v2), n_v2 = grid[i][i2] + (i2 != s1 ? v1 : v2);

            v1 = n_v1;
            v2 = n_v2;
            s1 = i1;
            s2 = i2;
        }

        return min(v1, v2);
    }
};
// end_submission
