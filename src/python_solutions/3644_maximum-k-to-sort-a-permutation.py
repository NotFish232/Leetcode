from common import *


# start_submission
class Solution:
    def sortPermutation(self, nums: List[int]) -> int:
        res = (1 << 32) - 1

        for i in range(len(nums)):
            if i != nums[i]:
                res &= i

        return res if res != (1 << 32) - 1 else 0


# end_submission
