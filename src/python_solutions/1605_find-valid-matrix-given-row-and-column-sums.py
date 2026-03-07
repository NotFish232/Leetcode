from common import *


# start_submission
class Solution:
    def restoreMatrix(self, rowSum: List[int], colSum: List[int]) -> List[List[int]]:
        m = len(rowSum)
        n = len(colSum)

        res = [[0] * n for _ in range(m)]

        cur_row_sums = [0] * m
        cur_col_sums = [0] * n

        for i in range(m):
            for j in range(n):
                el = min(rowSum[i] - cur_row_sums[i], colSum[j] - cur_col_sums[j])

                res[i][j] = el

                cur_row_sums[i] += el
                cur_col_sums[j] += el

        return res


# end_submission
