from common import *


# start_submission
class Solution:
    def largestPerimeter(self, nums: List[int]) -> int:
        n = len(nums)

        nums = sorted(nums, reverse=True)

        s = sum(nums)

        for i in range(n):
            if n - i == 2:
                return -1

            el = nums[i]

            s -= el

            if el < s:
                return s + el


# end_submission
