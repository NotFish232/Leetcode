from common import *


# start_submission
class Solution:
    def minInsertions(self, s: str) -> int:
        n = len(s)

        dp = [[float("inf")] * (n + 1) for _ in range(n + 1)]

        for i in range(n + 1):
            dp[i][0] = 0
            dp[0][i] = 0

        for i in range(n):
            for j in range(n):
                if s[i] == s[-(j + 1)]:
                    dp[i + 1][j + 1] = dp[i][j] + 1
                else:
                    dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j])

        return len(s) - dp[-1][-1]


# end_submission
