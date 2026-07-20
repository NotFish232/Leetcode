#include <algorithm>
#include <bits/stdc++.h>
#include <cstdlib>
#include <numeric>
#include <tuple>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Dsu {
private:
    vector<int> parents_, sizes_;

public:
    Dsu(int n) : parents_(n), sizes_(n, 1) {
        iota(parents_.begin(), parents_.end(), 0);
    }

    int find(int i) {
        if (i != parents_[i]) {
            parents_[i] = find(parents_[i]);
        }

        return parents_[i];
    }

    void unify(int i, int j) {
        int a = find(i), b = find(j);

        if (a == b) {
            return;
        }

        if (sizes_[a] > sizes_[b]) {
            swap(a, b);
        }

        parents_[a] = parents_[b];
        sizes_[b] += sizes_[a];
    }
};

class Solution {
public:
    int minCostConnectPoints(vector<vector<int>> &points) {
        vector<tuple<int, int, int>> edges;

        for (int i = 0; i < points.size(); ++i) {
            for (int j = 0; j < i; ++j) {
                edges.push_back({abs(points[i][0] - points[j][0]) + abs(points[i][1] - points[j][1]), i, j});
            }
        }

        sort(edges.begin(), edges.end(), [](const auto &a, const auto &b) {
            return get<0>(a) < get<0>(b);
        });

        Dsu dsu(points.size());
        int t = 0;

        for (const auto [c, i, j] : edges) {
            if (dsu.find(i) != dsu.find(j)) {
                t += c;
                dsu.unify(i, j);
            }
        }

        return t;
    }
};
// end_submission
