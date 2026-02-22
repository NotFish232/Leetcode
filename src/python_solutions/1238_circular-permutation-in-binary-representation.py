from common import *


# start_submission
class Solution:
    def circularPermutation(self, n: int, start: int) -> List[int]:
        x = [0, 1]

        for i in range(n - 1):
            x = x + [y | (1 << (i + 1)) for y in reversed(x)]

        idx = x.index(start)

        return x[idx:] + x[:idx]


# end_submission
