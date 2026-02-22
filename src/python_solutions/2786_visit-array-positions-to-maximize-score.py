from typing import *

# start_submission
class Solution:
    def maxScore(self, nums: List[int], x: int) -> int:
        dp = [[float("-inf")] * 2 for _ in range(len(nums))]
        dp[0][nums[0] % 2] = nums[0]

        for i in range(1, len(nums)):
            if nums[i] % 2 == 0:
                dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] - x) + nums[i]
                dp[i][1] = dp[i - 1][1]
            else:
                dp[i][0] = dp[i - 1][0]
                dp[i][1] = max(dp[i - 1][0] - x, dp[i - 1][1]) + nums[i]

        return max(dp[-1])


# end_submission
