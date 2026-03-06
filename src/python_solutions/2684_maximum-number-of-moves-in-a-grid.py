from common import *


# start_submission
class Solution:
    def maxMoves(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])

        dp = [[float("-inf")] * n for _ in range(m)]

        for i in range(m):
            dp[i][0] = 0

        for j in range(1, n):
            for i in range(m):
                if i > 0 and grid[i][j] > grid[i - 1][j - 1]:
                    dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + 1)

                if i + 1 < m and grid[i][j] > grid[i + 1][j - 1]:
                    dp[i][j] = max(dp[i][j], dp[i + 1][j - 1] + 1)

                if grid[i][j] > grid[i][j - 1]:
                    dp[i][j] = max(dp[i][j], dp[i][j - 1] + 1)

        return max(x for row in dp for x in row)


# end_submission
