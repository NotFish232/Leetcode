#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
private:
    vector<vector<int>> build_dists(vector<vector<int>> &grid) {
        int n = grid.size();

        vector<vector<int>> dists(n, vector<int>(n, INT_MAX));

        queue<pair<pair<int, int>, int>> q;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    q.push({{i, j}, 0});
                    dists[i][j] = 0;
                }
            }
        }

        const vector<pair<int, int>> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

        while (q.size() > 0) {
            auto el = q.front();
            q.pop();

            auto [x, y] = el.first;
            int dist = el.second;

            for (auto [d_x, d_y] : dirs) {
                int new_x = x + d_x, new_y = y + d_y;

                if (new_x >= 0 && new_x < n && new_y >= 0 && new_y < n && dists[new_x][new_y] == INT_MAX) {
                    dists[new_x][new_y] = dist + 1;
                    q.push({{new_x, new_y}, dist + 1});
                }
            }
        }

        return dists;
    }

    bool possible(vector<vector<int>> &dists, int v) {
        int n = dists.size();

        queue<pair<int, int>> q;
        vector<vector<bool>> seen(n, vector<bool>(n, false));

        seen[0][0] = true;

        if (dists[0][0] >= v) {
            q.push({0, 0});
        }

        const vector<pair<int, int>> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

        while (q.size() > 0) {
            auto [x, y] = q.front();
            q.pop();

            if (x == n - 1 && y == n - 1) {
                return true;
            }

            for (auto [d_x, d_y] : dirs) {
                int new_x = x + d_x, new_y = y + d_y;

                if (new_x >= 0 && new_x < n && new_y >= 0 && new_y < n && !seen[new_x][new_y] && dists[new_x][new_y] >= v) {
                    q.push({new_x, new_y});
                    seen[new_x][new_y] = true;
                }
            }
        }

        return false;
    }

public:
    int maximumSafenessFactor(vector<vector<int>> &grid) {
        int n = grid.size();

        vector<vector<int>> dists = build_dists(grid);

        int l = 0, r = n;
        int ans = -1;

        while (l <= r) {
            int m = l + (r - l) / 2;

            if (possible(dists, m)) {
                l = m + 1;
                ans = m;
            } else {
                r = m - 1;
            }
        }

        return ans;
    }
};
// end_submission
