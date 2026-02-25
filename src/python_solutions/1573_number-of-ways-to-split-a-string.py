from common import *


# start_submission
class Solution:
    def numWays(self, s: str) -> int:
        count = s.count("1")

        if count % 3 != 0:
            return 0

        tgt = count // 3

        if count == 0:
            return (((len(s) - 1) * (len(s) - 2)) // 2) % (10**9 + 7)

        count_1 = 0

        a = -1
        b = -1
        c = -1
        d = -1

        for i, ch in enumerate(s):
            if ch == "1":
                count_1 += 1

            if count_1 == tgt and a == -1:
                a = i

            if count_1 == tgt + 1 and b == -1:
                b = i

            if count_1 == 2 * tgt and c == -1:
                c = i

            if count_1 == 2 * tgt + 1 and d == -1:
                d = i

        return ((b - a) * (d - c)) % (10**9 + 7)


# end_submission
