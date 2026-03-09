from common import *


# start_submission
class Solution:
    def canSeePersonsCount(self, heights: List[int]) -> List[int]:
        n = len(heights)

        stk = []

        ans = [0] * n

        for i in reversed(range(n)):
            cnt = 0

            while len(stk) > 0 and heights[i] > stk[-1]:
                stk.pop()
                cnt += 1

            if len(stk) == 0:
                ans[i] = cnt
            else:
                ans[i] = cnt + 1

            stk.append(heights[i])

        return ans


# end_submission
