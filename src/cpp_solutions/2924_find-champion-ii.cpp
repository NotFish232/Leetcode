#include <vector>

using namespace std;

 // start_submission
class Solution {
public:
    int findChampion(int n, vector<vector<int>>& edges) {
        vector<int> incoming(n);

        for (const auto &x: edges) {
            ++incoming[x[1]];
        }

        vector<int> cand;
        for (int i = 0; i < n; ++i) {
            if (incoming[i] == 0) {
                cand.push_back(i);
            }
        }

        return cand.size() == 1 ? cand.front() : -1;
    }
};
 // end_submission
