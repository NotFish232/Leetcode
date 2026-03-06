from common import *


# start_submission
class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        m = len(points)
        n = len(points[0])

        dp = [[float("-inf")] * n for _ in range(m)]

        for j in range(n):
            dp[0][j] = points[0][j]

        for i in range(1, m):
            l_max = [0] * n
            r_max = [0] * n

            for j in range(n):
                l_max[j] = dp[i - 1][j]

                if j > 0:
                    l_max[j] = max(l_max[j], l_max[j - 1] - 1)

            for j in reversed(range(n)):
                r_max[j] = dp[i - 1][j]

                if j + 1 < n:
                    r_max[j] = max(r_max[j], r_max[j + 1] - 1)

            for j in range(n):
                dp[i][j] = points[i][j] + max(l_max[j], r_max[j])

        return max(dp[-1])


# end_submission
