from common import *


# start_submission
class Solution:
    def minSpaceWastedKResizing(self, nums: List[int], k: int) -> int:
        n = len(nums)

        dp = [[float("inf")] * (k + 1) for _ in range(n + 1)]

        cur_sum = 0
        cur_max = float("-inf")

        dp[0][0] = 0
        for i in range(n):
            cur_sum += nums[i]
            cur_max = max(cur_max, nums[i])

            dp[i + 1][0] = (i + 1) * cur_max - cur_sum

        for i in range(1, n + 1):
            for j in range(1, k + 1):
                cur_sum = 0
                cur_max = float("-inf")

                for l in reversed(range(i)):
                    cur_sum += nums[l]
                    cur_max = max(cur_max, nums[l])

                    dp[i][j] = min(dp[i][j], dp[l][j - 1] + (i - l) * cur_max - cur_sum)

        return dp[-1][-1]


# end_submission
