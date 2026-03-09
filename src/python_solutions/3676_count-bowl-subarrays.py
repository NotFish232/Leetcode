from common import *


# start_submission
class Solution:
    def bowlSubarrays(self, nums: List[int]) -> int:
        n = len(nums)

        l_maxes = [0] * (n + 1)
        r_maxes = [0] * (n + 1)

        for i in range(n):
            l_maxes[i + 1] = max(l_maxes[i], nums[i])
        for i in reversed(range(n)):
            r_maxes[i] = max(r_maxes[i + 1], nums[i])

        ans = 0

        for i in range(1, n - 1):
            if nums[i] < l_maxes[i] and nums[i] < r_maxes[i + 1]:
                ans += 1

        return ans


# end_submission
