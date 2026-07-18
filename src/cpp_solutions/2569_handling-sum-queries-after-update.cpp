#include <numeric>
#include <vector>

using namespace std;

// start_submission
class LazySTree {
private:
    int n_;
    vector<int> nodes_, lazy_;

    void build_(const vector<int> &v, int p, int l, int r) {
        if (l == r) {
            nodes_[p] = v[l];
            return;
        }

        int m = l + (r - l) / 2;
        build_(v, 2 * p, l, m);
        build_(v, 2 * p + 1, m + 1, r);

        nodes_[p] = nodes_[2 * p] + nodes_[2 * p + 1];
    }

    void apply_(int p, int l, int r) {
        nodes_[p] = r - l + 1 - nodes_[p];
        lazy_[p] ^= 1;
    }

    void push_(int p, int l, int r) {
        if (lazy_[p] == 0) {
            return;
        }

        int m = l + (r - l) / 2;

        apply_(2 * p, l, m);
        apply_(2 * p + 1, m + 1, r);

        lazy_[p] = 0;
    }

public:
    explicit LazySTree(const vector<int> &v) : n_(v.size()), nodes_(4 * n_), lazy_(4 * n_) {
        build_(v, 1, 0, n_ - 1);
    }

    void update_(int p, int i, int j, int l, int r) {
        if (i <= l && r <= j) {
            apply_(p, l, r);
            return;
        }

        push_(p, l, r);

        int m = l + (r - l) / 2;

        if (i <= m) {
            update_(2 * p, i, j, l, m);
        }
        if (j > m) {
            update_(2 * p + 1, i, j, m + 1, r);
        }

        nodes_[p] = nodes_[2 * p] + nodes_[2 * p + 1];
    }

    void update(int i, int j) {
        update_(1, i, j, 0, n_ - 1);
    }

    int query_(int p, int i, int j, int l, int r) {
        if (i <= l && r <= j) {
            return nodes_[p];
        }

        push_(p, l, r);

        int m = l + (r - l) / 2;

        int res = 0;

        if (i <= m) {
            res += query_(2 * p, i, j, l, m);
        }
        if (j > m) {
            res += query_(2 * p + 1, i, j, m + 1, r);
        }

        return res;
    }

    int query(int i, int j) {
        return query_(1, i, j, 0, n_ - 1);
    }
};
class Solution {
public:
    vector<long long> handleQuery(vector<int> &nums1, vector<int> &nums2, vector<vector<int>> &queries) {
        LazySTree s{nums1};
        long long total = accumulate(nums2.begin(), nums2.end(), 0LL);

        vector<long long> ans;

        for (const vector<int> &q : queries) {
            if (q[0] == 1) {
                s.update(q[1], q[2]);
            } else if (q[0] == 2) {
                total += 1LL * q[1] * s.query(0, nums1.size() - 1);
            } else if (q[0] == 3) {
                ans.push_back(total);
            }
        }

        return ans;
    }
};
// end_submission
