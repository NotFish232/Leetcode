#include <numeric>
#include <set>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Dsu {
private:
    vector<int> parents, sizes;

public:
    Dsu(int n) : parents(n), sizes(n, 1) {
        iota(parents.begin(), parents.end(), 0);
    }

    int find(int i) {
        if (i != parents[i]) {
            parents[i] = find(parents[i]);
        }

        return parents[i];
    }

    void unify(int i, int j) {
        int a = find(i), b = find(j);

        if (a == b) {
            return;
        }

        if (sizes[a] > sizes[b]) {
            swap(a, b);
        }

        parents[a] = b;
        sizes[b] += sizes[a];
    }
};

class Solution {
public:
    int numberOfComponents(vector<vector<int>> &properties, int k) {
        Dsu dsu(properties.size());

        vector<set<int>> p;
        for (const auto &x : properties) {
            p.push_back({x.begin(), x.end()});
        }

        for (int i = 0; i < p.size(); ++i) {
            for (int j = 0; j < i; ++j) {
                int cnt = 0;
                for (const int &x : p[i]) {
                    if (p[j].find(x) != p[j].end()) {
                        ++cnt;
                    }
                }

                if (cnt >= k) {
                    dsu.unify(i, j);
                }
            }
        }

        set<int> s;
        for (int i = 0; i < properties.size(); ++i) {
            s.insert(dsu.find(i));
        }

        return s.size();
    }
};
// end_submission
