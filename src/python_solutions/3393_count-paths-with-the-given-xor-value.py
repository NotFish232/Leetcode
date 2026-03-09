from common import *


# start_submission
class Solution:
    def countPathsWithXorValue(self, grid: List[List[int]], k: int) -> int:
        MOD = 10**9 + 7

        n = len(grid)
        m = len(grid[0])

        dp = [[[0] * 16 for _ in range(m)] for _ in range(n)]

        dp[0][0][grid[0][0]] = 1

        for i in range(n):
            for j in range(m):
                for l in range(16):
                    if i > 0:
                        dp[i][j][l ^ grid[i][j]] += dp[i - 1][j][l]
                        dp[i][j][l ^ grid[i][j]] %= MOD
                    if j > 0:
                        dp[i][j][l ^ grid[i][j]] += dp[i][j - 1][l]
                        dp[i][j][l ^ grid[i][j]] %= MOD

        return dp[n - 1][m - 1][k]


# end_submission
