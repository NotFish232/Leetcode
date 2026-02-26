#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    typedef long long LL;

    LL solve(int cur, int par, const vector<int> &values, unordered_map<int, vector<int>> &edge_dict) {
        LL child_vals = 0;
        bool has_child = false;
        for (const auto &e : edge_dict[cur]) {
            if (e == par) {
                continue;
            }
            has_child = true;
            child_vals += solve(e, cur, values, edge_dict);
        }

        return !has_child ? (LL)values[cur] : min((LL)values[cur], child_vals);
    }

    long long maximumScoreAfterOperations(vector<vector<int>> &edges, vector<int> &values) {
        int n = values.size();

        unordered_map<int, vector<int>> edge_dict;

        for (const auto &e : edges) {
            edge_dict[e[0]].push_back(e[1]);
            edge_dict[e[1]].push_back(e[0]);
        }

        LL sm = accumulate(values.begin(), values.end(), 0LL);

        return sm - solve(0, -1, values, edge_dict);
    }
};
// end_submission
