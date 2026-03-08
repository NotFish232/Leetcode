from common import *


# start_submission
class Solution:
    def sumFourDivisors(self, nums: List[int]) -> int:
        N = max(nums)

        def factor(x):
            factors = set()

            n = 1

            while n**2 <= x:
                if x % n == 0:
                    factors.add(x // n)
                    factors.add(n)

                n += 1

            return factors

        ans = 0

        for x in nums:
            factors = factor(x)

            if len(factors) == 4:
                ans += sum(factors)

        return ans


# end_submission
