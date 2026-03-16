from common import *


# start_submission
class Solution:
    def maximumLength(self, s: str) -> int:
        n = len(s)

        l = 1
        r = n

        ans = -1

        while l <= r:
            m = l + (r - l) // 2

            works = False

            ch = "z"
            count = 0

            c = [0] * 26

            for i in range(n):
                if s[i] == ch:
                    count += 1
                else:
                    count = 1
                    ch = s[i]

                if count >= m:
                    k = ord(ch) - ord("a")
                    c[k] += 1

                    if c[k] == 3:
                        works = True
                        break

            if works:
                ans = m
                l = m + 1
            else:
                r = m - 1

        return ans


# end_submission
