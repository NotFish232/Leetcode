from common import *


# start_submission
class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        n = len(nums)

        dp = [[float("inf")] * 4 for _ in range(n)]

        for i in range(1, 4):
            dp[0][i] = 1

        dp[0][nums[0]] = 0

        for i in range(1, n):
            for j in range(1, 4):
                dp[i][j] = dp[i - 1][j] + 1

            dp[i][nums[i]] = min(dp[i - 1][: nums[i] + 1])

        return min(dp[-1])


# end_submission
