from common import *


# start_submission
class Solution:
    def goodDaysToRobBank(self, security: List[int], time: int) -> List[int]:
        n = len(security)

        l = [1] * n
        r = [1] * n

        for i in range(1, len(security)):
            if security[i] <= security[i - 1]:
                l[i] = l[i - 1] + 1

        for i in reversed(range(0, len(security) - 1)):
            if security[i] <= security[i + 1]:
                r[i] = r[i + 1] + 1

        return [i for i in range(n) if l[i] > time and r[i] > time]


# end_submission
