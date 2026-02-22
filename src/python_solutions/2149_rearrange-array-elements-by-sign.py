from common import *


# start_submission
class Solution:
    def rearrangeArray(self, nums: List[int]) -> List[int]:
        n = len(nums)
        ans = [0] * n

        even_i = 0
        odd_i = 0

        for n in nums:
            if n >= 0:
                ans[2 * even_i] = n
                even_i += 1
            else:
                ans[2 * odd_i + 1] = n
                odd_i += 1

        return ans


# end_submission
