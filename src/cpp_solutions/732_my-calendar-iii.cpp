#include <algorithm>
#include <bits/stdc++.h>
#include <set>

using namespace std;

 // start_submission
class MyCalendarThree {
private: 
    multiset<int> starts_, ends_;
public:
    MyCalendarThree() {
        
    }
    
    int book(int startTime, int endTime) {
        starts_.insert(startTime);
        ends_.insert(endTime);


        int cnt = 0;
        int ans = 0;

        for (auto starts_it = starts_.begin(), ends_it = ends_.begin(); starts_it != starts_.end();  ++starts_it) {
            ++cnt;

            while (ends_it != ends_.end() && *ends_it <= *starts_it) {
                --cnt;
                ++ends_it;
            }

            ans = max(ans, cnt);
        }

        return ans;
    }
};

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * MyCalendarThree* obj = new MyCalendarThree();
 * int param_1 = obj->book(startTime,endTime);
 */
 // end_submission
