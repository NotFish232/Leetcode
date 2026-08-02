#include <algorithm>
#include <vector>

using namespace std;

// start_submission
class Fenwick {
private:
    vector<int> tree;
    int n;

    int lsb(int x) {
        return x & -x;
    }

public:
    Fenwick(int n) : tree(n + 1), n(n) {}

    void update(int i, int val) {
        for (; i <= n; i += lsb(i)) {
            tree[i] += val;
        }
    }

    int prefix(int i) {
        int res = 0;
        for (; i > 0; i -= lsb(i)) {
            res += tree[i];
        }

        return res;
    }
};

class Solution {
public:
    vector<int> countSmaller(vector<int> &nums) {
        vector<int> coords(nums);
        sort(coords.begin(), coords.end());

        coords.erase(unique(coords.begin(), coords.end()), coords.end());

        Fenwick t(coords.size());

        vector<int> res(nums.size());

        for (int i = nums.size() - 1; i >= 0; --i) {
            int c = lower_bound(coords.begin(), coords.end(), nums[i]) - coords.begin();

            res[i] = t.prefix(c);

            t.update(c + 1, 1);
        }

        return res;
    }
};
// end_submission
