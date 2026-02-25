from common import *


# start_submission
class Solution:
    def minimumPossibleSum(self, n: int, target: int) -> int:
        half = target // 2

        if n <= half:
            return (n * (n + 1) // 2) % (10**9 + 7)

        half_contrib = half * (half + 1) // 2

        remaining = n - half

        return (
            (target + remaining) * (target + remaining - 1) // 2
            - (target - 1) * target // 2
            + half_contrib
        ) % (10**9 + 7)


# end_submission
