#include <unordered_map>
#include <unordered_set>

using namespace std;

// start_submission
class FrequencyTracker {
    unordered_map<int, int> num_to_freq_;
    unordered_map<int, unordered_set<int>> freq_to_num_;

public:
    FrequencyTracker() {
    }

    void add(int number) {
        int &freq = num_to_freq_[number];
        ++freq;

        if (freq > 1) {
            unordered_set<int> &freq_entry = freq_to_num_[freq - 1];

            freq_entry.erase(number);
            if (freq_entry.empty()) {
                freq_to_num_.erase(freq - 1);
            }
        }
        freq_to_num_[freq].insert(number);
    }

    void deleteOne(int number) {
        auto it = num_to_freq_.find(number);

        if (it != num_to_freq_.end()) {
            int &freq = it->second;

            unordered_set<int> &freq_entry = freq_to_num_[freq];

            freq_entry.erase(number);
            if (freq_entry.empty()) {
                freq_to_num_.erase(freq);
            }

            --freq;

            if (freq == 0) {
                num_to_freq_.erase(it);
            } else {
                freq_to_num_[freq].insert(number);
            }
        }
    }

    bool hasFrequency(int frequency) {
        return freq_to_num_.find(frequency) != freq_to_num_.end();
    }
};

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * FrequencyTracker* obj = new FrequencyTracker();
 * obj->add(number);
 * obj->deleteOne(number);
 * bool param_3 = obj->hasFrequency(frequency);
 */
// end_submission
