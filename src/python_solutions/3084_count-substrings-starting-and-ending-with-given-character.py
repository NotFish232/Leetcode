from common import *


# start_submission
class Solution:
    def countSubstrings(self, s: str, c: str) -> int:
        cnt = 0
        ans = 0

        for ch in s:
            if ch == c:
                cnt += 1
                ans += cnt

        return ans


# end_submission
