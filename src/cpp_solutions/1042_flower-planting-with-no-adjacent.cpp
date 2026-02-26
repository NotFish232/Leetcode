#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    vector<int> gardenNoAdj(int n, vector<vector<int>> &paths) {
        vector<int> colors(n);

        map<int, vector<int>> edges;

        for (const auto &p : paths) {
            int a = p[0] - 1;
            int b = p[1] - 1;

            edges[a].push_back(b);
            edges[b].push_back(a);
        }

        for (int i = 0; i < n; ++i) {
            if (colors[i] != 0) {
                continue;
            }

            set<int> edge_colors;
            for (const auto &e : edges[i]) {
                edge_colors.insert(colors[e]);
            }

            for (int j = 1; j <= 4; ++j) {
                if (edge_colors.find(j) == edge_colors.end()) {
                    colors[i] = j;
                    break;
                }
            }
        }

        return colors;
    }
};
// end_submission
