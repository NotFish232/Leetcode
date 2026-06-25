#include <bits/stdc++.h>

using namespace std;

// start_submission
class Solution {
public:
    void build(int cur_sum, vector<int> &nums, set<int> &unused, vector<vector<int>> &out, int k, int n) {
        if (cur_sum > n) {
            return;
        }

        if (nums.size() == k) {
            if (cur_sum == n) {
                out.push_back(nums);
            }
            return;
        }

        vector<int> v(unused.begin(), unused.end());

        for (const int &x : v) {
            if (nums.size() > 0 && x > nums.back()) {
                break;
            }
            
            cur_sum += x;
            nums.push_back(x);
            unused.erase(x);

            build(cur_sum, nums, unused, out, k, n);

            cur_sum -= x;
            nums.pop_back();
            unused.insert(x);
        }
    }

    vector<vector<int>> combinationSum3(int k, int n) {
        vector<vector<int>> out;

        vector<int> nums;
        set<int> unused{1, 2, 3, 4, 5, 6, 7, 8, 9};

        build(0, nums, unused, out, k, n);

        return out;
    }
};
// end_submission
