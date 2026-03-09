from common import *


# start_submission
class Solution:
    def taskSchedulerII(self, tasks: List[int], space: int) -> int:
        prev = {}

        count = 0

        for task in tasks:
            if task in prev:
                count = max(count, prev[task] + space + 1)

            prev[task] = count

            count += 1

        return count


# end_submission
