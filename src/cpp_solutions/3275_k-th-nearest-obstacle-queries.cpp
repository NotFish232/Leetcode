#include <bits/stdc++.h>
#include <cstdlib>
#include <queue>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    vector<int> resultsArray(vector<vector<int>> &queries, int k) {
        auto cmp = [](const auto &a, const auto &b) {
            return abs(a.first) + abs(a.second) < abs(b.first) + abs(b.second);
        };

        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(cmp)> pq(cmp);

        vector<int> res;
        for (const auto &q : queries) {
            pq.push({q[0], q[1]});

            if (pq.size() > k) {
                pq.pop();
            }

            res.push_back(pq.size() == k ? (abs(pq.top().first) + abs(pq.top().second)) : -1);
        }

        return res;
    }
};
// end_submission
