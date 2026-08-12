#include <algorithm>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    int height(int cur, int p, vector<vector<int>> &adj) {
        int d = 0;

        for (int n : adj[cur]) {
            if (n == p) {
                continue;
            }

            d = max(d, height(n, cur, adj));
        }

        return d + 1;
    }

    long long helper(int cur, int p, int d, int h, vector<vector<int>> &adj, vector<int> &nums) {
        long long res = 0;

        for (const int &n : adj[cur]) {
            if (n == p) {
                continue;
            }
            res += helper(n, cur, d + 1, h, adj, nums);
        }

        return res + (long long)nums[cur] * (h - d + 1);
    }

    long long weightedSum(vector<int> &parent, vector<int> &nums) {
        int n = parent.size();

        vector<vector<int>> adj(n);

        for (int i = 1; i < n; ++i) {
            int j = parent[i];

            adj[i].push_back(j);
            adj[j].push_back(i);
        }

        return helper(0, -1, 1, height(0, -1, adj), adj, nums);
    }
};
// end_submission
