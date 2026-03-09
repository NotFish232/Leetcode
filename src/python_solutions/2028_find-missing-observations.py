from common import *


# start_submission
class Solution:
    def missingRolls(self, rolls: List[int], mean: int, n: int) -> List[int]:
        m = len(rolls)

        tgt = mean * (n + m) - sum(rolls)

        if 6 * n < tgt or tgt < n:
            return []

        ans = []

        while tgt > 0:
            d = tgt // (n - len(ans))

            tgt -= d

            ans.append(d)

        return ans


# end_submission
