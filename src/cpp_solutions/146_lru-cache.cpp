#include <bits/stdc++.h>
#include <iterator>
#include <list>
#include <unordered_map>
#include <utility>

using namespace std;

// start_submission
class LRUCache {
private:
    int capacity_;
    list<pair<int, int>> queue_;
    unordered_map<int, list<pair<int, int>>::iterator> map_;

public:
    LRUCache(int capacity) : capacity_(capacity) {}

    int get(int key) {
        auto it = map_.find(key);

        if (it == map_.end()) {
            return -1;
        }

        queue_.splice(queue_.end(), queue_, it->second);

        return it->second->second;
    }

    void put(int key, int value) {
        auto it = map_.find(key);

        if (it != map_.end()) {
            queue_.erase(it->second);
        }

        queue_.push_back({key, value});
        map_[key] = std::prev(queue_.end());

        if (queue_.size() > capacity_) {
            map_.erase(queue_.front().first);
            queue_.pop_front();
        }
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
// end_submission
