from common import *


# start_submission
class Solution:
    def subarrayGCD(self, nums: List[int], k: int) -> int:
        cnt = 0

        for i in range(len(nums)):
            gcd = 0
            for j in range(i, len(nums)):
                if gcd == 0:
                    gcd = nums[j]
                else:
                    gcd = math.gcd(gcd, nums[j])

                if gcd == k:
                    cnt += 1

        return cnt


# end_submission
