from common import *


# start_submission
class Solution:
    def makeStringsEqual(self, s: str, target: str) -> bool:
        return (
            any(x == "1" for x in s) and any(x == "1" for x in target)
        ) or s == target


# end_submission
