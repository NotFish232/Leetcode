#include <bits/stdc++.h>
#include <vector>

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

class BSTIterator {
private:
    vector<TreeNode *> stk;

public:
    BSTIterator(TreeNode *root) {
        while (root != nullptr) {
            stk.push_back(root);
            root = root->left;
        }
    }

    int next() {
        TreeNode *cur = stk.back();

        if (cur->right == nullptr) {
            stk.pop_back();
            return cur->val;
        }

        stk.back() = cur->right;

        while (stk.back()->left != nullptr) {
            stk.push_back(stk.back()->left);
        }

        return cur->val;
    }

    bool hasNext() {
        return stk.size() > 0;
    }
};

/**
 * Your BSTIterator object will be instantiated and called as such:
 * BSTIterator* obj = new BSTIterator(root);
 * int param_1 = obj->next();
 * bool param_2 = obj->hasNext();
 */
// end_submission
