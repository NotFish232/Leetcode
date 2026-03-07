from common import *


# start_submission
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        s = []

        for t in tokens:
            if t not in "+-*/":
                s.append(int(t))
            else:
                b = s.pop()
                a = s.pop()

                if t == "+":
                    s.append(a + b)
                elif t == "-":
                    s.append(a - b)
                elif t == "*":
                    s.append(a * b)
                else:
                    sign = (a < 0) ^ (b < 0)
                    res = abs(a) // abs(b)
                    if sign:
                        res *= -1
                    s.append(res)

        return s[0]


# end_submission
