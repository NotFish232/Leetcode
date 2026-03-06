from common import *


# start_submission
class Solution:
    def checkOnesSegment(self, s: str) -> bool:
        return all(x == "1" for x in s[s.index("1") : s.rindex("1") + 1])


# end_submission
