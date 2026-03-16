from common import *


# start_submission
class Solution:
    def maximumSwap(self, num: int) -> int:
        digits = list(str(num))

        n = len(digits)

        stk = []

        k, l = -1, -1

        for i in reversed(range(n)):
            if len(stk) > 0 and digits[i] > digits[stk[-1]]:
                stk.pop()

            if len(stk) > 0 and digits[stk[-1]] > digits[i]:
                k, l = i, stk[-1]

            if len(stk) == 0 or digits[i] > digits[stk[-1]]:
                stk.append(i)

        if k != -1 and l != -1:
            digits[k], digits[l] = digits[l], digits[k]

        return int("".join(digits))


# end_submission
