#include <iterator>
#include <map>
#include <set>
#include <tuple>

using namespace std;

// start_submission

struct Comparator {
    int n;

public:
    bool operator()(const tuple<int, int> &a, const tuple<int, int> &b) const {
        auto [a1, a2] = a;
        auto [b1, b2] = b;

        int a_size = a1 == 0 || a2 == n - 1 ? a2 - a1 : (a2 - a1) / 2;
        int b_size = b1 == 0 || b2 == n - 1 ? b2 - b1 : (b2 - b1) / 2;

        return a_size != b_size ? a_size < b_size : b1 < a1;
    }
};

class ExamRoom {
private:
    set<tuple<int, int>, Comparator> sorted_ranges_;
    map<int, int> adj_ranges_;
    int n_;

public:
    ExamRoom(int n) : sorted_ranges_(Comparator{n}), n_(n) {
        sorted_ranges_.insert({0, n - 1});
        adj_ranges_[0] = n - 1;
    }

    int seat() {
        auto it = std::prev(sorted_ranges_.end());
        auto [r1, r2] = *it;

        sorted_ranges_.erase(it);
        adj_ranges_.erase(r1);

        int tgt = r1 + (r2 - r1) / 2;

        if (r1 == 0) {
            tgt = 0;
        } else if (r2 == n_ - 1) {
            tgt = n_ - 1;
        }

        if (tgt > r1) {
            sorted_ranges_.insert({r1, tgt - 1});
            adj_ranges_[r1] = tgt - 1;
        }
        if (tgt < r2) {
            sorted_ranges_.insert({tgt + 1, r2});
            adj_ranges_[tgt + 1] = r2;
        }

        return tgt;
    }

    void leave(int p) {
        auto it = adj_ranges_.find(p + 1);

        int l = p, r = p;

        if (it != adj_ranges_.begin()) {
            auto prev_it = std::prev(it);
            if (prev_it->second == p - 1) {
                l = prev_it->first;

                sorted_ranges_.erase({prev_it->first, prev_it->second});
                adj_ranges_.erase(prev_it);

                it = adj_ranges_.find(p + 1);
            }
        }
        if (it != adj_ranges_.end()) {
            r = it->second;

            sorted_ranges_.erase({it->first, it->second});
            adj_ranges_.erase(it);
        }

        adj_ranges_[l] = r;
        sorted_ranges_.insert({l, r});
    }
};

/**
 * Your ExamRoom object will be instantiated and called as such:
 * ExamRoom* obj = new ExamRoom(n);
 * int param_1 = obj->seat();
 * obj->leave(p);
 */
// end_submission
