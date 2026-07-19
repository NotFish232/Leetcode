#include <vector>

using namespace std;

// start_submission
class RLEIterator {
private:
    vector<int> encoding_;
    int idx_;

public:
    RLEIterator(vector<int> &encoding) : encoding_(encoding), idx_{0} {
    }

    int next(int n) {
        n -= 1;
        while (idx_ < encoding_.size()) {
            while (idx_ < encoding_.size() && encoding_[idx_] == 0) {
                idx_ += 2;
            }

            if (idx_ >= encoding_.size() || n == 0) {
                break;
            }

            if (n >= encoding_[idx_]) {
                n -= encoding_[idx_];
                encoding_[idx_] = 0;
            } else {
                encoding_[idx_] -= n;
                n = 0;
            }
        }

        if (idx_ < encoding_.size()) {
            --encoding_[idx_];
            return encoding_[idx_ + 1];
        } else {
            return -1;
        }
    }
};

/**
 * Your RLEIterator object will be instantiated and called as such:
 * RLEIterator* obj = new RLEIterator(encoding);
 * int param_1 = obj->next(n);
 */
// end_submission
