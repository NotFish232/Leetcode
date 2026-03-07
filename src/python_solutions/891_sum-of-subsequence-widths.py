from common import *


# start_submission
class Solution:
    def sumSubseqWidths(self, nums: List[int]) -> int:
        mod = 10**9 + 7

        n = len(nums)

        nums = sorted(nums)

        ans = 0

        for i in range(n):
            ans += pow(2, i, mod) * nums[i]
            ans -= pow(2, (n - i - 1), mod) * nums[i]
            ans %= mod

        return ans


# end_submission
