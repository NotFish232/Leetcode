#include <algorithm>
#include <vector>

using namespace std;

// start_submission
class CustomStack {
private:
    int size_;
    vector<int> stk_, increments_;

public:
    CustomStack(int maxSize) : size_(maxSize), increments_(size_) {
    }

    void push(int x) {
        if (stk_.size() < size_) {
            stk_.push_back(x);
        }
    }

    int pop() {
        if (stk_.size() == 0) {
            return -1;
        }

        int val = stk_.back();
        stk_.pop_back();

        int n = stk_.size();

        int increment = increments_[n];
        increments_[n] = 0;

        val += increment;

        if (n > 0) {
            increments_[n - 1] += increment;
        }

        return val;
    }

    void increment(int k, int val) {
        if (stk_.size() > 0) {
            increments_[min(k, (int)stk_.size()) - 1] += val;
        }
    }
};

/**
 * Your CustomStack object will be instantiated and called as such:
 * CustomStack* obj = new CustomStack(maxSize);
 * obj->push(x);
 * int param_2 = obj->pop();
 * obj->increment(k,val);
 */
// end_submission
