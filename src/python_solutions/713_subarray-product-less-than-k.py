from common import *


# start_submission
class Solution:
    def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
        n = len(nums)

        l = 0

        cur_prod = 1

        ans = 0

        for r in range(n):
            cur_prod *= nums[r]

            while cur_prod >= k and l < n:
                cur_prod /= nums[l]
                l += 1

            ans += max(0, r - l + 1)

        return ans


# end_submission
