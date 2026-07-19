#include <bits/stdc++.h>
#include <vector>

using namespace std;

// start_submission
class NumMatrix {
private:
    int n_, m_;
    vector<vector<int>> p_sum_;

public:
    NumMatrix(vector<vector<int>> &matrix) : n_(matrix.size()),
                                             m_(matrix[0].size()),
                                             p_sum_(n_ + 1, vector<int>(m_ + 1, 0)) {
        for (int i = 0; i < n_; ++i) {
            for (int j = 0; j < m_; ++j) {
                p_sum_[i + 1][j + 1] = matrix[i][j] + p_sum_[i + 1][j] + p_sum_[i][j + 1] - p_sum_[i][j];
            }
        }
    }

    int sumRegion(int row1, int col1, int row2, int col2) {
        return p_sum_[row2 + 1][col2 + 1] - p_sum_[row1][col2 + 1] - p_sum_[row2 + 1][col1] + p_sum_[row1][col1];
    }
};

/**
 * Your NumMatrix object will be instantiated and called as such:
 * NumMatrix* obj = new NumMatrix(matrix);
 * int param_1 = obj->sumRegion(row1,col1,row2,col2);
 */
// end_submission
