#include <algorithm>
#include <deque>
#include <iterator>
#include <queue>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class Router {
private:
    int memory_limit_;
    queue<tuple<int, int, int>> queue_;
    set<tuple<int, int, int>> seen_packets_;
    unordered_map<int, deque<int>> dest_to_packets_;

public:
    Router(int memoryLimit) : memory_limit_(memoryLimit) {
    }

    bool addPacket(int source, int destination, int timestamp) {
        if (seen_packets_.find({source, destination, timestamp}) != seen_packets_.end()) {
            return false;
        }

        queue_.push({source, destination, timestamp});
        seen_packets_.insert({source, destination, timestamp});
        dest_to_packets_[destination].push_back(timestamp);

        if (queue_.size() > memory_limit_) {
            tuple<int, int, int> first = queue_.front();
            queue_.pop();
            seen_packets_.erase(first);
            dest_to_packets_[get<1>(first)].pop_front();
        }

        return true;
    }

    vector<int> forwardPacket() {
        if (queue_.size() == 0) {
            return {};
        }

        tuple<int, int, int> data = queue_.front();
        queue_.pop();
        seen_packets_.erase(data);
        dest_to_packets_[get<1>(data)].pop_front();

        return {get<0>(data), get<1>(data), get<2>(data)};
    }

    int getCount(int destination, int startTime, int endTime) {
        const deque<int> &packets = dest_to_packets_[destination];

        auto s = lower_bound(packets.begin(), packets.end(), startTime);
        auto e = upper_bound(packets.begin(), packets.end(), endTime);

        return std::distance(s, e);
    }
};

/**
 * Your Router object will be instantiated and called as such:
 * Router* obj = new Router(memoryLimit);
 * bool param_1 = obj->addPacket(source,destination,timestamp);
 * vector<int> param_2 = obj->forwardPacket();
 * int param_3 = obj->getCount(destination,startTime,endTime);
 */
// end_submission
