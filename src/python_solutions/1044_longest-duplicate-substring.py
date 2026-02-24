from common import *


# start_submission
class Solution:
    def longestDupSubstring(self, s: str) -> str:
        l = 1
        r = len(s) + 1

        ans = ""

        while l <= r:
            m = (l + r) // 2

            seen = set()
            found = False

            for i in range(len(s) - m + 1):
                subs = s[i : i + m]

                if subs in seen:
                    ans = subs
                    found = True
                    break

                seen.add(subs)

            if found:
                l = m + 1
            else:
                r = m - 1

        return ans


# end_submission
