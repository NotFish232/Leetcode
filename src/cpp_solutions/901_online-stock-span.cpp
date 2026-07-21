#include <tuple>
#include <vector>

using namespace std;

 // start_submission
class StockSpanner {
vector<tuple<int, int>> stk_;

public:
    StockSpanner() {
        
    }
    
    int next(int price) {
        int cnt = 1;
        
        while (stk_.size() > 0 && get<0>(stk_.back()) <= price)  {
            cnt += get<1>(stk_.back());
            stk_.pop_back();
        }

        stk_.push_back({price, cnt});

        return cnt;
    }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */
 // end_submission
