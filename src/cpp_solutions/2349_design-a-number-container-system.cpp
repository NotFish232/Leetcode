#include <bits/stdc++.h>
#include <set>
#include <unordered_map>

using namespace std;

 // start_submission
class NumberContainers {
private:
    unordered_map<int, int> idx_to_num_;
    unordered_map<int, set<int>> num_to_idxs_;
public:
    NumberContainers() {
        
    }
    
    void change(int index, int number) {
        auto it = idx_to_num_.find(index);

        if (it != idx_to_num_.end()) {
            int old_num = it->second;

            num_to_idxs_[old_num].erase(index);

            if (num_to_idxs_[old_num].size() == 0) {
                num_to_idxs_.erase(old_num);
            }
        }

        idx_to_num_[index] = number;
        num_to_idxs_[number].insert(index);
    }
    
    int find(int number) {
        auto it = num_to_idxs_.find(number);
        
        return it == num_to_idxs_.end() ? -1 : *it->second.begin();
    }
};

/**
 * Your NumberContainers object will be instantiated and called as such:
 * NumberContainers* obj = new NumberContainers();
 * obj->change(index,number);
 * int param_2 = obj->find(number);
 */
 // end_submission
