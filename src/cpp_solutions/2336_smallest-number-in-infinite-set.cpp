#include <bits/stdc++.h>
#include <set>

using namespace std;

// start_submission
class SmallestInfiniteSet {
private:
    set<int> numbers_;
    int cur_;

public:
    SmallestInfiniteSet() : cur_{1} {
    }

    int popSmallest() {
        if (numbers_.size() == 0) {
            return cur_++;
        } else {
            int num = *numbers_.begin();
            numbers_.erase(numbers_.begin());

            return num;
        }
    }

    void addBack(int num) {
        if (num < cur_) {
            numbers_.insert(num);
        }
    }
};

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * SmallestInfiniteSet* obj = new SmallestInfiniteSet();
 * int param_1 = obj->popSmallest();
 * obj->addBack(num);
 */
// end_submission
