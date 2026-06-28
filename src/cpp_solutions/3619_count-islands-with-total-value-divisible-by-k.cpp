#include <bits/stdc++.h>

using namespace std;

// start_submission
class DSU {
private:
    vector<int> parents;
    vector<int> sizes;

public:
    DSU(int n) : parents(n), sizes(n, 1) {
        iota(parents.begin(), parents.end(), 0);
    }

    int find(int i) {
        if (i != parents[i]) {
            parents[i] = find(parents[i]);
        }

        return parents[i];
    }

    void unio(int i, int j) {
        int i_rep = find(i);
        int j_rep = find(j);

        if (i_rep == j_rep) {
            return;
        }

        if (sizes[i_rep] > sizes[j_rep]) {
            swap(i_rep, j_rep);
        }

        parents[i_rep] = j_rep;
        sizes[j_rep] += sizes[i_rep];
    }
};

class Solution {
public:
    int countIslands(vector<vector<int>> &grid, int k) {
        int n = grid.size(), m = grid[0].size();

        DSU dsu(n * m);

        const vector<pair<int, int>> dirs{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] > 0) {
                    for (const auto &[d_i, d_j] : dirs) {
                        int n_i = i + d_i, n_j = j + d_j;

                        if (0 <= n_i && n_i < n && 0 <= n_j && n_j < m && grid[n_i][n_j] > 0) {
                            dsu.unio(i * m + j, n_i * m + n_j);
                        }
                    }
                }
            }
        }

        unordered_map<int, long long> mp;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] > 0) {
                    mp[dsu.find(i * m + j)] += grid[i][j];
                }
            }
        }

        return accumulate(mp.begin(), mp.end(), 0, [&](const int &acc, const auto &p) {
            return acc + (p.second % k == 0);
        });
    }
};
// end_submission
