from common import *


# start_submission
class Solution:
    def minSwaps(self, s: str) -> int:
        n = len(s)

        p0 = 0
        p1 = 0

        for i in range(n):
            if i % 2 == 0:
                if s[i] == "1":
                    p0 += 1
            else:
                if s[i] == "0":
                    p1 += 1

        p2 = 0
        p3 = 0

        for i in range(n):
            if i % 2 == 0:
                if s[i] == "0":
                    p2 += 1
            else:
                if s[i] == "1":
                    p3 += 1

        ans = -1

        if p0 == p1:
            ans = p0
        if p2 == p3:
            ans = p2 if ans == -1 else min(ans, p2)

        return ans


# end_submission
