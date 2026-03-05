from common import *


# start_submission
class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        n = len(values)

        dp = [[float("inf")] * n for _ in range(n)]

        for i in range(n):
            for j in range(n):
                if j - i < 2:
                    dp[i][j] = 0
                if j - i == 2:
                    dp[i][j] = values[i] * values[i + 1] * values[i + 2]

        for s in range(3, n):
            for i in range(n - s):
                j = i + s
                for k in range(i + 1, j):
                    dp[i][j] = min(
                        dp[i][j],
                        dp[i][k] + dp[k][j] + values[i] * values[k] * values[j],
                    )

        return dp[0][n - 1]


# end_submission
