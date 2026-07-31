#include <bits/stdc++.h>
#include <vector>

using namespace std;

 // start_submission
 class Fenwick {
private:
    int n_;
    vector<long long> tree_;


    int lsb(int x) {
        return x & -x;
    }

public:
    Fenwick(int n): n_(n), tree_(n_ + 1, 0) {}

    void update(int index, int val) {
        for (; index <= n_; index += lsb(index)) {
            tree_[index] += val;
        }
    }

    long long prefix(int index) {
        long long res = 0;

        for (; index > 0; index -= lsb(index)) {
            res += tree_[index];
        }

        return res;
    }
 };
 
class NumArray {
private:
    vector<int> nums;
    Fenwick tree;
public:
    NumArray(vector<int>& nums): nums(nums), tree(nums.size()) {
        for (int i = 0; i < nums.size(); ++i) {
            tree.update(i + 1, nums[i]);
        }
    }
    
    void update(int index, int val) {
        tree.update(index + 1, val - nums[index]);
        nums[index] = val;\
    }
    
    int sumRange(int left, int right) {
        return tree.prefix(right + 1) - tree.prefix(left);
    }
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * obj->update(index,val);
 * int param_2 = obj->sumRange(left,right);
 */
 // end_submission
