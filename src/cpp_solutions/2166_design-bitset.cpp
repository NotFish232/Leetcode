#include <string>
#include <vector>

using namespace std;

 // start_submission
class Bitset {
private:
    vector<bool> bits_;
    int one_count_;
    bool flipped_;
    
public:
    Bitset(int size): bits_(size, false), one_count_{0}, flipped_{false} {
        
    }
    
    void fix(int idx) {
        if (!bits_[idx] ^ flipped_) {
            bits_[idx] = !bits_[idx];
            ++one_count_;
        }
    }
    
    void unfix(int idx) {
        if (bits_[idx] ^ flipped_) {
            bits_[idx] = !bits_[idx];
            --one_count_;
        }
    }
    
    void flip() {
        flipped_ = !flipped_;
        one_count_ = bits_.size() - one_count_;
    }
    
    bool all() {
        return one_count_ == bits_.size();
    }
    
    bool one() {
        return one_count_ > 0;
    }
    
    int count() {
        return one_count_;
    }
    
    string toString() {
        string s;

        for (const bool &b: bits_) {
            s += b ^ flipped_ ? '1': '0';
        }

        return s;
    }
};

/**
 * Your Bitset object will be instantiated and called as such:
 * Bitset* obj = new Bitset(size);
 * obj->fix(idx);
 * obj->unfix(idx);
 * obj->flip();
 * bool param_4 = obj->all();
 * bool param_5 = obj->one();
 * int param_6 = obj->count();
 * string param_7 = obj->toString();
 */
 // end_submission
