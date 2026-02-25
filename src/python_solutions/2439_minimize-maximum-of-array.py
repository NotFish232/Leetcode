from common import *


# start_submission
class Solution:
    def minimizeArrayValue(self, nums: List[int]) -> int:
        l = nums[0]
        r = max(nums)

        ans = -1

        while l <= r:
            m = (l + r) // 2

            delta = m - nums[0]

            possible = True

            for i in range(1, len(nums)):
                if m >= nums[i] - delta:
                    delta = m - nums[i] + delta
                else:
                    possible = False
                    break

            if possible:
                r = m - 1
                ans = m
            else:
                l = m + 1

        return ans


# end_submission
