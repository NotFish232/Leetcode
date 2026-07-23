#include <numeric>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class FindSumPairs {
private:
    unordered_map<int, int> val_freqs_;
    vector<int> nums1_, nums2_;

public:
    FindSumPairs(vector<int> &nums1, vector<int> &nums2) : nums1_{nums1}, nums2_{nums2} {
        for (const int &x : nums2) {
            ++val_freqs_[x];
        }
    }

    void add(int index, int val) {
        int old = nums2_[index];

        --val_freqs_[old];

        if (val_freqs_[old] == 0) {
            val_freqs_.erase(old);
        }

        nums2_[index] += val;
        ++val_freqs_[old + val];
    }

    int count(int tot) {
        return accumulate(nums1_.begin(), nums1_.end(), 0, [this, tot](const int &acc, const int &el) {
            auto it = val_freqs_.find(tot - el);

            if (it == val_freqs_.end()) {
                return acc;
            }

            return acc + it->second;
        });
    }
};

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * FindSumPairs* obj = new FindSumPairs(nums1, nums2);
 * obj->add(index,val);
 * int param_2 = obj->count(tot);
 */
// end_submission
