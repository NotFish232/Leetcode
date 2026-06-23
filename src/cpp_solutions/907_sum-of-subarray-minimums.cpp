#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    int sumSubarrayMins(vector<int> &arr) {
        int n = arr.size();

        vector<int> stk;

        vector<int> pre;
        for (int i = 0; i < n; ++i) {
            while (stk.size() > 0 && arr[i] < arr[stk.back()]) {
                stk.pop_back();
            }

            pre.push_back(stk.size() > 0 ? stk.back() : -1);
            stk.push_back(i);
        }
        stk.clear();

        vector<int> post;
        for (int i = n - 1; i >= 0; --i) {
            while (stk.size() > 0 && arr[i] <= arr[stk.back()]) {
                stk.pop_back();
            }

            post.push_back(stk.size() > 0 ? stk.back() : n);
            stk.push_back(i);
        }

        long long ans = 0;
        for (int i = 0; i < n; ++i) {
            ans += (long long)arr[i] * (long long)(post[n - i - 1] - i) * (long long)(i - pre[i]);
            ans %= 1'000'000'007;
        }

        return ans;
    }
};
// end_submission
