from common import *


# start_submission
class Solution:
    def lastRemaining(self, n: int) -> int:
        first = 1
        step = 1

        dir = True

        while n > 1:
            if dir:
                first += step
            else:
                if n & 1 == 1:
                    first += step

            step *= 2
            n >>= 1

            dir = not dir

        return first


# end_submission
