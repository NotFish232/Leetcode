#include <queue>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
bool comparator(const tuple<int, int, int> &a, const tuple<int, int, int> &b) {
    return get<1>(a) == get<1>(b) ? get<2>(a) > get<2>(b) : get<1>(a) > get<1>(b);
}

class FreqStack {
private:
    // (element, freq, idx)
    set<tuple<int, int, int>, decltype(&comparator)> data_;
    unordered_map<int, int> freq_map_;

    int idx_;

public:
    FreqStack() : idx_{0}, data_(comparator) {
    }

    void push(int val) {
        ++freq_map_[val];

        data_.insert({val, freq_map_[val], ++idx_});
    }

    int pop() {
        int val = get<0>(*data_.begin());

        --freq_map_[val];

        data_.erase(data_.begin());

        return val;
    }
};

/**
 * Your FreqStack object will be instantiated and called as such:
 * FreqStack* obj = new FreqStack();
 * obj->push(val);
 * int param_2 = obj->pop();
 */
// end_submission
