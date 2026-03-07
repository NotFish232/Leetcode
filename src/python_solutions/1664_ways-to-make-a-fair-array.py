from common import *


# start_submission
class Solution:
    def waysToMakeFair(self, nums: List[int]) -> int:
        n = len(nums)

        even_sum = [0] * (n + 1)
        odd_sum = [0] * (n + 1)
        total_sum = 0

        for i in range(n):
            even_sum[i + 1] = even_sum[i]
            odd_sum[i + 1] = odd_sum[i]

            if i % 2 == 0:
                even_sum[i + 1] += nums[i]
            else:
                odd_sum[i + 1] += nums[i]

            total_sum += nums[i]

        ans = 0

        for i in range(n):
            if total_sum - nums[i] == 2 * (even_sum[i] + odd_sum[n] - odd_sum[i + 1]):
                ans += 1

        return ans


# end_submission
