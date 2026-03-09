from common import *


# start_submission
class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)

        stk = []

        ans = []

        for i in reversed(range(n)):
            while len(stk) > 0 and temperatures[i] >= stk[-1][0]:
                stk.pop()

            ans.append(stk[-1][1] - i if len(stk) > 0 else 0)

            stk.append((temperatures[i], i))

        return ans[::-1]


# end_submission
