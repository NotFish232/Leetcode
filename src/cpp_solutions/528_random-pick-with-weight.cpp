#include <algorithm>
#include <cstdlib>
#include <numeric>
#include <vector>

using namespace std;

 // start_submission
class Solution {
private:
    vector<float> w_;
    
public:
    Solution(vector<int>& w): w_(w.size()) {
        partial_sum(w.begin(), w.end(), w_.begin());
    }
    
    int pickIndex() {
        float r = ((float)rand() / RAND_MAX) * w_[w_.size() - 1];

        return lower_bound(w_.begin(), w_.end(), r) - w_.begin();
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(w);
 * int param_1 = obj->pickIndex();
 */
 // end_submission
