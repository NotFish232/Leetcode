from common import *


# start_submission
class Solution:
    def removeDuplicates(self, s: str, k: int) -> str:
        stk = []

        for ch in s:
            if len(stk) > 0 and stk[-1][0] == ch:
                if stk[-1][1] + 1 == k:
                    stk.pop()
                else:
                    stk[-1] = (ch, stk[-1][1] + 1)
            else:
                stk.append((ch, 1))

        return "".join(a * b for a, b in stk)


# end_submission
