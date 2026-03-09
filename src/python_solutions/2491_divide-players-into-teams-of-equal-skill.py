from common import *


# start_submission
class Solution:
    def dividePlayers(self, skill: List[int]) -> int:
        n = len(skill)

        s = sum(skill)

        if s % (n // 2) != 0:
            return -1

        tgt = s // (n // 2)

        m = {}
        ans = 0

        for s in skill:
            if tgt - s in m:
                ans += s * (tgt - s)

                m[tgt - s] -= 1
                if m[tgt - s] == 0:
                    del m[tgt - s]
            else:
                if s not in m:
                    m[s] = 0
                m[s] += 1

        return ans if len(m) == 0 else -1


# end_submission
