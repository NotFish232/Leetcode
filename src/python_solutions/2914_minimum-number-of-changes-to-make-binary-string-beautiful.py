from common import *


# start_submission
class Solution:
    def minChanges(self, s: str) -> int:
        return sum(s[2 * i] != s[2 * i + 1] for i in range(len(s) // 2))


# end_submission
