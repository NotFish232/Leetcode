#include <algorithm>
#include <limits>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

// start_submission

class Solution {
public:
    int helper(TreeNode *cur, int &ans) {
        if (cur == nullptr) {
            return 0;
        }

        int l_val = max(helper(cur->left, ans), 0), r_val = max(helper(cur->right, ans), 0);

        ans = max(ans, l_val + r_val + cur->val);

        return max(l_val, r_val) + cur->val;
    }

    int maxPathSum(TreeNode *root) {
        int ans = numeric_limits<int>::min();
        helper(root, ans);

        return ans;
    }
};
// end_submission
