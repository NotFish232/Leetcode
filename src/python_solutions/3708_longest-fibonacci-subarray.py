from common import *


# start_submission
class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        n = len(nums)

        cur = 2
        ans = float("-inf")

        for i in range(2, n):
            if nums[i] == nums[i - 1] + nums[i - 2]:
                cur += 1
            else:
                cur = 2

            ans = max(ans, cur)

        return ans


# end_submission
