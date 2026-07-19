#include <bits/stdc++.h>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    string encode(string w) {
        unordered_map<char, char> m;
        vector<char> chars;
        char cur = 'a';

        for (const char &ch : w) {
            if (m.find(ch) == m.end()) {
                m[ch] = cur;
                ++cur;
            }

            chars.push_back(m[ch]);
        }

        return {chars.begin(), chars.end()};
    }

    vector<string> findAndReplacePattern(vector<string> &words, string pattern) {
        string p = encode(pattern);

        vector<string> v;

        for (const string &word : words) {
            if (encode(word) == p) {
                v.push_back(word);
            }
        }

        return v;
    }
};
// end_submission
