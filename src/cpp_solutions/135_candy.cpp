#include <vector>
#include <deque>
#include <utility>

using namespace std;

// start_submission
class Solution {
public:
    int candy(vector<int>& ratings) {
        int n = ratings.size();

        vector<vector<int>> adj(n);
        for (int i = 0; i < n; ++i) {
            if (i > 0 && ratings[i] < ratings[i - 1]) {
                adj[i].push_back(i - 1);
            }
            if (i + 1 < n && ratings[i] < ratings[i + 1]) {
                adj[i].push_back(i + 1);
            }
        }

        vector<int> in(n);
        for (auto &x: adj) {
            for (auto &y: x) {
                ++in[y];
            }
        }

        deque<pair<int, int>> s;
        for (int i = 0; i < n; ++i) {
            if (in[i] == 0) {
                s.push_back({i, 1});
            }
        }

        int cnt = 0;

        while (s.size() > 0) {
            auto [a, b] = s.front();
            s.pop_front();

            cnt += b;

            for (int x: adj[a]) {
                --in[x];

                if (in[x] == 0) {
                    s.push_back({x, b + 1});
                }
            }
        }

        return cnt;
    }
};
// end_submission