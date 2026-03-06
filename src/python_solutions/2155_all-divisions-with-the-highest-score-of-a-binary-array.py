from common import *


# start_submission
class Solution:
    def maxScoreIndices(self, nums: List[int]) -> List[int]:
        n = len(nums)

        zero = [0] * (n + 1)

        for i in range(n):
            zero[i + 1] = zero[i] + (nums[i] == 0)

        max_cnt = max(zero[i] + n - i - (zero[n] - zero[i]) for i in range(n + 1))

        ans = []

        for i in range(n + 1):
            if zero[i] + n - i - (zero[n] - zero[i]) == max_cnt:
                ans.append(i)

        return ans


# end_submission
