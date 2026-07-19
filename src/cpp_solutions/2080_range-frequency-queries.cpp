#include <algorithm>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class RangeFreqQuery {
private:
    unordered_map<int, vector<int>> idxs_;

public:
    RangeFreqQuery(vector<int> &arr) {
        for (int i = 0; i < arr.size(); ++i) {
            idxs_[arr[i]].push_back(i);
        }
    }

    int query(int left, int right, int value) {
        auto it = idxs_.find(value);

        if (it == idxs_.end()) {
            return 0;
        }

        return upper_bound(it->second.begin(), it->second.end(), right) - lower_bound(it->second.begin(), it->second.end(), left);
    }
};

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * RangeFreqQuery* obj = new RangeFreqQuery(arr);
 * int param_1 = obj->query(left,right,value);
 */
// end_submission
