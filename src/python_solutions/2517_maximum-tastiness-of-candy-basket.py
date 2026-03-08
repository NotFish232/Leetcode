from common import *


# start_submission
class Solution:
    def maximumTastiness(self, price: List[int], k: int) -> int:
        n = len(price)

        price.sort()

        l = 0
        r = price[-1]

        ans = -1

        while l <= r:
            m = (l + r) // 2

            count = 1
            i = price[0]

            while count < k:
                j = bisect_left(price, i + m)

                if j == n:
                    break

                count += 1
                i = price[j]

            if count == k:
                ans = m
                l = m + 1
            else:
                r = m - 1

        return ans


# end_submission
