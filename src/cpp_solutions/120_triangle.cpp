#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int minimumTotal(vector<vector<int>> &triangle) {
        int n = triangle.size();

        vector<int> cur_row(triangle[0]);

        for (int i = 2; i <= n; ++i) {
            vector<int> new_row(i, INT_MAX / 2);

            for (int j = 0; j < i; ++j) {
                if (j + 1 < i) {
                    new_row[j] = min(new_row[j], triangle[i - 1][j] + cur_row[j]);
                }
                if (j > 0) {
                    new_row[j] = min(new_row[j], triangle[i - 1][j] + cur_row[j - 1]);
                }
            }

            cur_row = new_row;
        }

        return *min_element(cur_row.begin(), cur_row.end());
    }
};
// end_submission
