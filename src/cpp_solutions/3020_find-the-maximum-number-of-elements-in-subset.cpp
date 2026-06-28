#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int maximumLength(vector<int> &nums) {
        unordered_map<unsigned long long, int> m;
        for (const int &x : nums) {
            m[x]++;
        }

        vector<pair<unsigned long long, int>> v(m.begin(), m.end());
        sort(v.begin(), v.end());

        int ans = 0;

        for (const auto &p : v) {
            unsigned long long k = p.first;
            int cnt = 1;

            while (m[k] >= 2 && m.find(k * k) != m.end() && (k != 1 || m[k] >= 3)) {
                cnt += 2;
                m[k] -= 2;
                k *= k;
            }

            ans = max(ans, cnt);
        }

        return ans;
    }
};
// end_submission
