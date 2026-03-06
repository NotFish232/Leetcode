from common import *


# start_submission
class Solution:
    def maxIncreaseKeepingSkyline(self, grid: List[List[int]]) -> int:
        row_maxes = [max(x) for x in grid]
        col_maxes = [
            max(grid[i][j] for i in range(len(grid))) for j in range(len(grid[0]))
        ]

        cnt = 0

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                cnt += min(row_maxes[i], col_maxes[j]) - grid[i][j]

        return cnt


# end_submission
