#include <set>
#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Solution {

public:
    vector<pair<int, int>> dirs{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    void dfs(vector<vector<char>> &board,
             unordered_set<string> &res,
             set<int> &used,
             int i, int j,
             string &cur,
             unordered_set<string> &good,
             unordered_set<string> &wds) {
        if (good.find(cur) == good.end()) {
            return;
        }

        if (wds.find(cur) != wds.end()) {
            wds.erase(cur);
            res.insert(cur);
        }

        if (cur.size() == 10) {
            return;
        }

        for (auto &d : dirs) {
            int a = i + d.first, b = j + d.second;

            if (a >= 0 && a < board.size() && b >= 0 && b < board[0].size() && used.find(a * board[0].size() + b) == used.end()) {
                used.insert(a * board[0].size() + b);
                cur += board[a][b];

                dfs(board, res, used, a, b, cur, good, wds);

                cur.pop_back();
                used.erase(a * board[0].size() + b);
            }
        }
    }
    vector<string> findWords(vector<vector<char>> &board, vector<string> &words) {
        unordered_set<string> res;

        set<int> used;
        string s;

        unordered_set<string> good;
        for (string &w : words) {
            for (int i = 1; i <= w.size(); ++i) {
                good.insert(w.substr(0, i));
            }
        }
        good.insert("");

        unordered_set<string> wds(words.begin(), words.end());

        for (int i = 0; i < board.size(); ++i) {
            for (int j = 0; j < board[0].size(); ++j) {
                used.clear();

                s = "";
                s.push_back(board[i][j]);
                used.insert(i * board[0].size() + j);

                dfs(board, res, used, i, j, s, good, wds);
            }
        }

        return vector(res.begin(), res.end());
    }
};

// end_submission
