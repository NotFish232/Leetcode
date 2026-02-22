from common import *


# start_submission
class Solution:
    def numSteps(self, s: str) -> int:
        cnt = 0

        carry = 0

        for ch in reversed(s[1:]):
            if ch == "1":
                if carry == 1:
                    cnt += 1
                else:
                    cnt += 2
                    carry = 1
            else:
                if carry == 1:
                    cnt += 2
                else:
                    cnt += 1

        return cnt + (carry == 1)


# end_submission
