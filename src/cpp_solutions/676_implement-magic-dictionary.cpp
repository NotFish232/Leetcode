#include <set>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class MagicDictionary {
private:
    unordered_map<string, set<string>> seen_;

public:
    MagicDictionary() {
    }

    void buildDict(vector<string> dictionary) {
        seen_.clear();

        for (const string &word : dictionary) {
            vector<char> chars{word.begin(), word.end()};

            for (char &ch : chars) {
                char tmp = ch;
                ch = '#';

                seen_[{chars.begin(), chars.end()}].insert(word);

                ch = tmp;
            }
        }
    }

    bool search(string searchWord) {
        vector<char> chars{searchWord.begin(), searchWord.end()};

        for (char &ch : chars) {
            char tmp = ch;
            ch = '#';

            auto it = seen_.find({chars.begin(), chars.end()});

            if (it != seen_.end() && (*it->second.begin() != searchWord || it->second.size() > 1)) {
                return true;
            }

            ch = tmp;
        }

        return false;
    }
};

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * MagicDictionary* obj = new MagicDictionary();
 * obj->buildDict(dictionary);
 * bool param_2 = obj->search(searchWord);
 */
// end_submission
