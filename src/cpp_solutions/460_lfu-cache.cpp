#include <iterator>
#include <list>
#include <unordered_map>

using namespace std;

// start_submission
class LFUCache {
private:
    using iter = list<int>::iterator;

    struct entry {
        int val;
        int freq;
        iter it;
    };

    int capacity_, min_freq_;

    unordered_map<int, entry> data_;
    unordered_map<int, list<int>> freq_bins_;

    void _touch(int key) {
        entry &node = data_.at(key);

        list<int> &bin = freq_bins_.at(node.freq);
        bin.erase(node.it);

        if (bin.empty()) {
            freq_bins_.erase(node.freq);

            if (node.freq == min_freq_) {
                ++min_freq_;
            }
        }

        ++node.freq;

        freq_bins_[node.freq].push_back(key);

        node.it = prev(freq_bins_[node.freq].end());
    }

public:
    LFUCache(int capacity) : capacity_{capacity}, min_freq_{0} {
    }

    int get(int key) {
        if (data_.find(key) == data_.end()) {
            return -1;
        } else {
            _touch(key);

            return data_.at(key).val;
        }
    }

    void put(int key, int value) {
        if (capacity_ == 0) {
            return;
        }
        
        auto it = data_.find(key);

        if (it == data_.end()) {
            if (data_.size() == capacity_) {
                list<int> &bin = freq_bins_.at(min_freq_);

                int victim = bin.front();

                bin.erase(bin.begin());

                if (bin.empty()) {
                    freq_bins_.erase(min_freq_);
                }

                data_.erase(victim);
            }

            min_freq_ = 1;

            list<int> &bin = freq_bins_[min_freq_];

            bin.push_back(key);
            data_[key] = entry{value, 1, prev(bin.end())};
        } else {
            data_[key].val = value;
            _touch(key);
        }
    }
};

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// end_submission
