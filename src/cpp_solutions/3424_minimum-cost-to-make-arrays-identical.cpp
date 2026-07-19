#include <algorithm>
#include <cstdlib>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    long long minCost(vector<int> &arr, vector<int> &brr, long long k) {
        int n = arr.size();

        long long cost_1 = 0;
        for (int i = 0; i < n; ++i) {
            cost_1 += abs(arr[i] - brr[i]);
        }

        unordered_map<int, int> a_freq, b_freq;

        for (int i = 0; i < n; ++i) {
            ++a_freq[arr[i]];
            ++b_freq[brr[i]];
        }

        for (auto &[k, v] : a_freq) {
            v = min(v, b_freq[k]);
            b_freq[k] = v;
        }
        for (auto &[k, v] : b_freq) {
            v = min(v, a_freq[k]);
        }

        vector<int> new_a, new_b;

        for (int i = 0; i < n; ++i) {
            if (a_freq[arr[i]] > 0) {
                --a_freq[arr[i]];
            } else {
                new_a.push_back(arr[i]);
            }

            if (b_freq[brr[i]] > 0) {
                --b_freq[brr[i]];
            } else {
                new_b.push_back(brr[i]);
            }
        }

        sort(new_a.begin(), new_a.end());
        sort(new_b.begin(), new_b.end());

        long long cost_2 = k;
        for (int i = 0; i < new_a.size(); ++i) {
            cost_2 += abs(new_a[i] - new_b[i]);
        }

        return min(cost_1, cost_2);
    }
};
// end_submission
