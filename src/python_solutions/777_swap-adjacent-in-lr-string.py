from common import *


# start_submission
class Solution:
    def canTransform(self, start: str, result: str) -> bool:
        n = len(result)

        if start.count("X") != result.count("X"):
            return False

        l = []
        r = []

        for i in range(n):
            if start[i] != "X":
                l.append((i, start[i]))
            if result[i] != "X":
                r.append((i, result[i]))

        for i in range(len(l)):
            i1, c1 = l[i]
            i2, c2 = r[i]

            if c1 != c2:
                return False

            if c1 == "R" and i2 < i1:
                return False
            if c1 == "L" and i2 > i1:
                return False

        return True


# end_submission
