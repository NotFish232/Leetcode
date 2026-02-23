from common import *


# start_submission
class Solution:
    def lastInteger(self, n: int) -> int:
        if n == 1:
            return n

        ans = 1
        cnt = 1

        while True:
            n = (n + 1) // 2

            if n == 1:
                return ans

            cnt *= 2

            if n % 2 == 0:
                ans += cnt

            n = (n + 1) // 2

            if n == 1:
                return ans

            cnt *= 2


# end_submission
