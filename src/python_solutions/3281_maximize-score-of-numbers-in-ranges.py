from common import *


# start_submission
class Solution:
    def maxPossibleScore(self, start: List[int], d: int) -> int:
        n = len(start)
        start = sorted(start)

        l = 0
        r = max(start) + d

        ans = -1

        while l <= r:
            m = (l + r) // 2

            cur = start[0]

            good = True

            for i in range(1, n):
                cur = max(cur + m, start[i])

                if cur > start[i] + d:
                    good = False
                    break

            if good:
                ans = m
                l = m + 1
            else:
                r = m - 1

        return ans


# end_submission
