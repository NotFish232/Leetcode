#include <vector>

using namespace std;

// start_submission
class MinStack {
private:
    vector<int> stk_, mins_;

public:
    MinStack() {
    }

    void push(int value) {
        stk_.push_back(value);
        if ((mins_.size() > 0 && value <= mins_.back()) || mins_.size() == 0) {
            mins_.push_back(value);
        }
    }

    void pop() {
        if (mins_.size() > 0 && mins_.back() == stk_.back()) {
            mins_.pop_back();
        }
        stk_.pop_back();
    }

    int top() {
        return stk_.back();
    }

    int getMin() {
        return mins_.back();
    }
};

/**
 * Your MinStack object will be instantiated and called as such:
 * MinStack* obj = new MinStack();
 * obj->push(value);
 * obj->pop();
 * int param_3 = obj->top();
 * int param_4 = obj->getMin();
 */
// end_submission
