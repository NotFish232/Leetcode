#include <functional>
#include <queue>
#include <vector>

using namespace std;

// start_submission
class MedianFinder {
private:
    priority_queue<int> max_heap_;                            // bottom n/2
    priority_queue<int, vector<int>, greater<int>> min_heap_; // top n/2
    
public:
    MedianFinder() {
    }

    void addNum(int num) {
        if (!max_heap_.empty() && num > max_heap_.top()) {
            min_heap_.push(num);
        } else {
            max_heap_.push(num);
        }

        if (min_heap_.size() > max_heap_.size()) {
            max_heap_.push(min_heap_.top());
            min_heap_.pop();
        }

        if (max_heap_.size() > min_heap_.size() + 1) {
            min_heap_.push(max_heap_.top());
            max_heap_.pop();
        }
    }

    double findMedian() {
        if (max_heap_.size() != min_heap_.size()) {
            return max_heap_.top();
        }

        return (double)(max_heap_.top() + min_heap_.top()) / 2;
    }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */
// end_submission
