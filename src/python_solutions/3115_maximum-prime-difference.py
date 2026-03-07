from common import *


# start_submission
class Solution:
    @staticmethod
    def is_prime(n: int) -> int:
        if n == 1:
            return False

        i = 2
        while i * i <= n:
            if n % i == 0:
                return False

            i += 1

        return True

    def maximumPrimeDifference(self, nums: List[int]) -> int:
        idxs = [i for i, x in enumerate(nums) if Solution.is_prime(x)]

        return max(idxs) - min(idxs)


# end_submission
