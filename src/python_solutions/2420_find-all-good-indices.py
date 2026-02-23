from common import *


# start_submission
class Solution:
    def goodIndices(self, nums: List[int], k: int) -> List[int]:
        n = len(nums)

        l = [1] * n
        r = [1] * n

        for i in range(1, len(nums)):
            if nums[i] <= nums[i - 1]:
                l[i] = l[i - 1] + 1

        for i in reversed(range(len(nums) - 1)):
            if nums[i] <= nums[i + 1]:
                r[i] = r[i + 1] + 1

        return [i for i in range(k, n - k) if l[i - 1] >= k and r[i + 1] >= k]


# end_submission
