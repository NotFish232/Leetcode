from common import *


# start_submission
class Solution:
    def longestSquareStreak(self, nums: List[int]) -> int:
        nums = sorted(nums, reverse=True)

        dp = {}

        for x in nums:
            if x**2 in dp:
                dp[x] = dp[x**2] + 1
            else:
                dp[x] = 1

        ans = max(dp.values())

        return ans if ans != 1 else -1


# end_submission
