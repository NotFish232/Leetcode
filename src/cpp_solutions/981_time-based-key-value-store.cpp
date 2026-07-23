#include <iterator>
#include <set>
#include <string>
#include <tuple>
#include <unordered_map>

using namespace std;

 // start_submission
class TimeMap {
private:
    unordered_map<string, set<tuple<int, string>>> data_;
    
public:
    TimeMap() {
        
    }
    
    void set(string key, string value, int timestamp) {
        data_[key].insert({timestamp, value});
    }
    
    string get(string key, int timestamp) {        
        auto it = data_.find(key);

        if (it == data_.end()) {
            return "";
        }
        
        auto cand = it->second.upper_bound({timestamp + 1, ""});

        return cand == it->second.begin() ? "" : get<1>(*prev(cand));
    }
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * TimeMap* obj = new TimeMap();
 * obj->set(key,value,timestamp);
 * string param_2 = obj->get(key,timestamp);
 */
 // end_submission
