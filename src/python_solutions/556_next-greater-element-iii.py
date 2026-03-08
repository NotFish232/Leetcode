from common import *


# start_submission
class Solution:
    def nextGreaterElement(self, n: int) -> int:
        s = list(map(int, str(n)))
        k = len(s)

        r_maxes = [float("-inf")] * k

        for i in reversed(range(k)):
            if i + 1 < k:
                r_maxes[i] = r_maxes[i + 1]

            r_maxes[i] = max(r_maxes[i], s[i])

        for i in reversed(range(len(s) - 1)):
            if r_maxes[i + 1] > s[i]:
                for j in reversed(range(len(s))):
                    if s[j] > s[i]:
                        s[i], s[j] = s[j], s[i]

                        s[i + 1 :] = sorted(s[i + 1 :])

                        ans = int("".join(map(str, s)))

                        return ans if ans < 2**31 else -1

        return -1


# end_submission
