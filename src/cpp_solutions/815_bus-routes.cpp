#include <map>
#include <set>
#include <vector>
#include <queue>

using namespace std;

// start_submission
class Solution {
public:
    int numBusesToDestination(vector<vector<int>> &routes, int source, int target) {
        if (source == target) {
            return 0;
        }
        // map number to each route that contains idx
        map<int, set<int>> g;
        for (int i = 0; i < routes.size(); ++i) {
            for (int x : routes[i]) {
                g[x].insert(i);
            }
        }

        int n = routes.size();

        map<int, int> m;

        queue<int> q;

        for (int x : g[source]) {
            m[x] = 1;
            q.push(x);
        }

        while (q.size() > 0) {
            int x = q.front();
            q.pop();

            if (g[target].count(x)) {
                return m[x];
            }

            for (int j : routes[x]) {
                for (int k : g[j]) {
                    if (m.find(k) == m.end()) {
                        m[k] = m[x] + 1;
                        q.push(k);
                    }
                }
            }
        }

        return -1;
    }
};

// end_submission
