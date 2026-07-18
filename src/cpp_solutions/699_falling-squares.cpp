#include <algorithm>
#include <vector>

using namespace std;

// start_submission
class LazySTree {
private:
    int n_;
    vector<long long> nodes_, lazy_;

    void apply_(int p, int l, int r, long long val) {
        nodes_[p] = val;
        lazy_[p] = val;
    }

    void push_(int p, int l, int r) {
        if (lazy_[p] == 0) {
            return;
        }

        int m = l + (r - l) / 2;

        long long add = lazy_[p];
        apply_(2 * p, l, m, add);
        apply_(2 * p + 1, m + 1, r, add);
        lazy_[p] = 0;
    }

public:
    LazySTree(int n) : n_(n), nodes_(4 * n_), lazy_(4 * n_) {}

    void update_(int p, int l, int r, int i, int j, long long val) {
        if (i <= l && r <= j) {
            apply_(p, l, r, val);
            return;
        }

        push_(p, l, r);

        int m = l + (r - l) / 2;

        if (i <= m) {
            update_(2 * p, l, m, i, j, val);
        }
        if (j > m) {
            update_(2 * p + 1, m + 1, r, i, j, val);
        }

        nodes_[p] = max(nodes_[2 * p], nodes_[2 * p + 1]);
    }

    void update(int i, int j, long long add) {
        update_(1, 0, n_ - 1, i, j, add);
    }

    long long query_(int p, int l, int r, int i, int j) {
        if (i <= l && r <= j) {
            return nodes_[p];
        }

        push_(p, l, r);

        int m = l + (r - l) / 2;

        long long ans = 0;

        if (i <= m) {
            ans = max(ans, query_(2 * p, l, m, i, j));
        }
        if (j > m) {
            ans = max(ans, query_(2 * p + 1, m + 1, r, i, j));
        }

        return ans;
    }

    long long query(int i, int j) {
        return query_(1, 0, n_ - 1, i, j);
    }
};

class Solution {
public:
    vector<int> fallingSquares(vector<vector<int>> &positions) {
        vector<long long> coords;

        for (const vector<int> &square : positions) {
            long long left = square[0];
            long long right = left + (long long)square[1] - 1;

            coords.push_back(left);
            coords.push_back(right);
        }

        sort(coords.begin(), coords.end());
        coords.erase(unique(coords.begin(), coords.end()), coords.end());

        LazySTree s_tree(coords.size());

        vector<int> ans;

        for (const vector<int> &square : positions) {
            long long left = square[0];
            long long right = left + (long long)square[1] - 1;

            long long cl = lower_bound(coords.begin(), coords.end(), left) - coords.begin();
            long long cr = lower_bound(coords.begin(), coords.end(), right) - coords.begin();

            s_tree.update(cl, cr, s_tree.query(cl, cr) + square[1]);

            ans.push_back(s_tree.query(0, coords.size() - 1));
        }

        return ans;
    }
};
// end_submission
