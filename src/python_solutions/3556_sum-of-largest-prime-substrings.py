from common import *


# start_submission
class Solution:
    @staticmethod
    def is_prime(x: int) -> bool:
        if x == 1:
            return False

        for i in range(2, math.floor(math.sqrt(x)) + 1):
            if x % i == 0:
                return False

        return True

    def sumOfLargestPrimes(self, s: str) -> int:
        a = set()

        for i in range(len(s)):
            for j in range(i + 1, len(s) + 1):
                x = int(s[i:j])

                if Solution.is_prime(x):
                    a.add(x)

        return sum(sorted(a)[-3:])


# end_submission
