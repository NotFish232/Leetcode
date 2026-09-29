#include <deque>
#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    vector<double> calcEquation(vector<vector<string>> &equations, vector<double> &values, vector<vector<string>> &queries) {
        map<string, vector<string>> adj;
        map<pair<string, string>, double> value_map;
        set<string> vars;

        for (auto &a : equations) {
            adj[a[0]].push_back(a[1]);
            adj[a[1]].push_back(a[0]);

            vars.insert(a[0]);
            vars.insert(a[1]);
        }

        for (int i = 0; i < equations.size(); ++i) {
            value_map[{equations[i][0], equations[i][1]}] = values[i];
            value_map[{equations[i][1], equations[i][0]}] = 1.0 / values[i];
        }

        auto find = [&](const string &s, const string &e) {
            if (s == e) {
                return vars.find(s) != vars.end() ? 1.0 : -1.0;
            }

            map<string, string> prev;

            deque<string> q;

            q.push_back(s);
            prev[s] = "";

            bool found = false;

            while (q.size() > 0) {
                string cur = q.front();
                q.pop_front();

                if (cur == e) {
                    found = true;
                    break;
                }

                for (string &nbr : adj[cur]) {
                    if (prev.find(nbr) == prev.end()) {
                        prev[nbr] = cur;
                        q.push_back(nbr);
                    }
                }
            }

            if (!found) {
                return -1.0;
            }

            double ans = 1.0;
            string cur = e;

            while (true) {
                string p = prev[cur];

                if (p == "") {
                    break;
                }

                ans *= value_map[{cur, p}];

                cur = p;
            }

            return ans;
        };

        vector<double> res;

        for (auto &q : queries) {
            double ans = find(q[0], q[1]);

            res.push_back(1.0 / ans);
        }

        return res;
    }
};
// end_submission
