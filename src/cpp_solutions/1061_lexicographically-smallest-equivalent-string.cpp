#include <algorithm>
#include <bits/stdc++.h>
#include <map>
#include <numeric>
#include <string>
#include <utility>
#include <vector>

using namespace std;

// start_submission
class Dsu {
private:
    vector<int> parents, sizes;

public:
    Dsu(int n) : parents(n), sizes(n, 1) {
        iota(parents.begin(), parents.end(), 0);
    }

    int find(int i) {
        if (i != parents[i]) {
            parents[i] = find(parents[i]);
        }

        return parents[i];
    }

    void unify(int i, int j) {
        int a = find(i), b = find(j);

        if (a == b) {
            return;
        }

        if (sizes[a] > sizes[b]) {
            swap(a, b);
        }

        parents[a] = b;
        sizes[b] += sizes[a];
    }
};

class Solution {
public:
    string smallestEquivalentString(string s1, string s2, string baseStr) {
        Dsu dsu(26);

        for (int i = 0; i < s1.size(); ++i) {
            int a = s1[i] - 'a', b = s2[i] - 'a';

            dsu.unify(a, b);
        }

        map<int, vector<int>> m;

        for (int i = 0; i < 26; ++i) {
            m[dsu.find(i)].push_back(i);
        }

        map<int, int> m2;
        for (const auto &[k, v] : m) {
            int min = *min_element(v.begin(), v.end());
            for (const int &x : v) {
                m2[x] = min;
            }
        }

        vector<char> out;
        for (const char &ch : baseStr) {
            out.push_back(m2[ch - 'a'] + 'a');
        }

        return {out.begin(), out.end()};
    }
};
// end_submission
