from common import *


# start_submission
class Solution:
    def getSumAbsoluteDifferences(self, nums: List[int]) -> List[int]:
        n = len(nums)

        p_sum = [0] * (n + 1)

        for i in reversed(range(n)):
            p_sum[i] = p_sum[i + 1] + nums[i]

        l_sum = 0

        ans = []

        for i in range(n):
            ans.append(i * nums[i] - l_sum + p_sum[i + 1] - (n - i - 1) * nums[i])

            l_sum += nums[i]

        return ans


# end_submission
