
#include <algorithm>
#include <limits>
#include <set>
#include <vector>
using namespace std;

 // start_submission
class Solution {
public:
    int maximalNetworkRank(int n, vector<vector<int>>& roads) {
        vector<set<int>> adj(n);

        for (const auto &road: roads) {
            int a = road[0], b = road[1];

            adj[a].insert(b);
            adj[b].insert(a);
        }

        int ans = numeric_limits<int>::min();

        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n; ++j) {
                ans = max(ans, (int)adj[i].size() + (int)adj[j].size() - (adj[i].find(j) != adj[i].end()));
            }
        }

        return ans;
    }
};
 // end_submission
