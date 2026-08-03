#include <algorithm>
#include <bits/stdc++.h>
#include <cmath>
#include <numbers>
#include <vector>

using namespace std;

// start_submission
class Solution {
public:
    vector<double> internalAngles(vector<int> &sides) {
        sort(sides.begin(), sides.end());

        if (sides[2] >= sides[0] + sides[1]) {
            return {};
        }

        vector<double> angles;
        for (int i = 0; i < 3; ++i) {
            int a = sides[i], b = sides[(i + 1) % 3], c = sides[(i + 2) % 3];

            double C = acos((double)(a * a + b * b - c * c) / (2 * a * b)) * 180.0 / numbers::pi;

            angles.push_back(C);
        }

        sort(angles.begin(), angles.end());

        return angles;
    }
};
// end_submission
