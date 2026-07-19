#include <memory>
#include <string>
#include <unordered_map>

using namespace std;

// start_submission
template <typename T>
class TrieNode {
private:
    std::unordered_map<T, std::unique_ptr<TrieNode>> children_;
    T val_;

public:
    TrieNode(T val) : val_(val) {}

    void add_child(T val) {
        children_[val] = std::make_unique<TrieNode<T>>(val);
    }

    TrieNode<T> *get_child(T val) {
        auto it = children_.find(val);

        if (it == children_.end()) {
            return nullptr;
        }

        return it->second.get();
    }
};

class Trie {
private:
    unique_ptr<TrieNode<char>> root_;

public:
    Trie() : root_(std::make_unique<TrieNode<char>>('\0')) {
    }

    void insert(string word) {
        TrieNode<char> *cur = root_.get();

        for (const char &ch : word) {
            if (cur->get_child(ch) == nullptr) {
                cur->add_child(ch);
            }

            cur = cur->get_child(ch);
        }

        cur->add_child('\0');
    }

    bool search(string word) {
        TrieNode<char> *cur = root_.get();

        for (const char &ch: word) {
            cur = cur->get_child(ch);

            if (cur == nullptr) {
                return false;
            }
        }

        return cur->get_child('\0') != nullptr;
    }

    bool startsWith(string prefix) {
        TrieNode<char> *cur = root_.get();

        for (const char &ch: prefix) {
            cur = cur->get_child(ch);

            if (cur == nullptr) {
                return false;
            }
        }

        return true;
    }
};

/**
 * Your Trie object will be instantiated and called as such:
 * Trie* obj = new Trie();
 * obj->insert(word);
 * bool param_2 = obj->search(word);
 * bool param_3 = obj->startsWith(prefix);
 */
// end_submission
