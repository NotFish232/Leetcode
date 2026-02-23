from common import *


# start_submission
class Solution:
    def maximumRemovals(self, s: str, p: str, removable: List[int]) -> int:
        l = 0
        r = len(removable)

        ans = -1

        while l <= r:
            m = (l + r) // 2

            r_set = set(removable[:m])

            p_idx = 0
            good = False

            for i, ch in enumerate(s):
                if i not in r_set and ch == p[p_idx]:
                    p_idx += 1

                if p_idx == len(p):
                    good = True
                    break

            if good:
                l = m + 1
                ans = m
            else:
                r = m - 1

        return ans


# end_submission
