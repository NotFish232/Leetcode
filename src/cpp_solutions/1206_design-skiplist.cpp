#include <algorithm>
#include <array>
#include <cstddef>
#include <random>
#include <vector>

// start_submission
template <typename T>
struct SkipListNode {
    T val;
    std::vector<SkipListNode<T> *> next;
};

class Skiplist {
private:
    static constexpr std::size_t max_levels = 32;

    using level_traversal = std::array<SkipListNode<int> *, max_levels>;
    using node = SkipListNode<int>;

    node *head_;
    std::size_t cur_max_level_;
    std::mt19937 gen_;
    std::bernoulli_distribution dist_;

    level_traversal _traverse(int tgt) {
        level_traversal traversal;
        traversal.fill(head_);

        node *cur = head_;

        for (int level = cur_max_level_ - 1; level >= 0; --level) {
            while (cur->next[level] != nullptr && cur->next[level]->val < tgt) {
                cur = cur->next[level];
            }
            traversal[level] = cur;
        }

        return traversal;
    }

public:
    Skiplist()
        : head_(new node(-1, std::vector<node *>(max_levels, nullptr))),
          cur_max_level_{0},
          gen_(std::random_device()()),
          dist_(0.5) {
    }

    ~Skiplist() {
        for (SkipListNode<int> *cur = head_; cur != nullptr;) {
            SkipListNode<int> *next = cur->next[0];
            delete cur;
            cur = next;
        }
    }

    bool search(int target) {
        level_traversal traversal = _traverse(target);
        node *next = traversal[0]->next[0];

        return next != nullptr && next->val == target;
    }

    void add(int num) {
        level_traversal traversal = _traverse(num);

        node *n = new node(num, {});

        std::size_t level = 0;

        for (; level < max_levels && level <= cur_max_level_ && (level == 0 || dist_(gen_)); ++level) {
            node *next = traversal[level]->next[level];
            traversal[level]->next[level] = n;
            n->next.push_back(next);
        }

        cur_max_level_ = std::max(cur_max_level_, level);
    }

    bool erase(int num) {
        level_traversal traversal = _traverse(num);

        node *cand = traversal[0]->next[0];

        if (cand == nullptr || cand->val != num) {
            return false;
        }

        for (std::size_t level = 0; level < cur_max_level_ && traversal[level]->next[level] == cand; ++level) {
            traversal[level]->next[level] = cand->next[level];
        }

        delete cand;

        return true;
    }
};

/**
 * Your Skiplist object will be instantiated and called as such:
 * Skiplist* obj = new Skiplist();
 * bool param_1 = obj->search(target);
 * obj->add(num);
 * bool param_3 = obj->erase(num);
 */
// end_submission
