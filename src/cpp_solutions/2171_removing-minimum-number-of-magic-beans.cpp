#include <bits/stdc++.h>
#include <climits>

using namespace std;

// start_submission
typedef long long LL;

class Solution {
public:
    long long minimumRemoval(vector<int> &beans) {
        int n = beans.size();

        sort(beans.begin(), beans.end());

        vector<LL> r_sum(n + 1);
        for (int i = beans.size() - 1; i >= 0; --i) {
            r_sum[i] = r_sum[i + 1] + (LL)beans[i];
        }

        LL l_sum = 0;

        LL best_ans = LLONG_MAX;

        for (int i = 0; i < n; ++i) {
            LL cur_ans = l_sum + r_sum[i + 1] - (LL)(n - i - 1) * (LL)beans[i];

            best_ans = min(best_ans, cur_ans);

            l_sum += (LL)beans[i];
        }

        return best_ans;
    }
};
// end_submission
