from common import *


# start_submission
class Solution:
    def maxDistance(self, arrays: List[List[int]]) -> int:
        mins = defaultdict(lambda: [])
        maxes = defaultdict(lambda: [])

        for i, a in enumerate(arrays):
            mins[a[0]].append(i)
            maxes[a[-1]].append(i)

        min_val = min(mins.keys())
        max_val = max(maxes.keys())

        if (
            len(mins[min_val]) > 1
            or len(maxes[max_val]) > 1
            or mins[min_val][0] != maxes[max_val][0]
        ):
            return max_val - min_val

        second_min_val = min(v for v in mins.keys() if v != min_val)
        second_max_val = max(v for v in maxes.keys() if v != max_val)

        return max(max_val - second_min_val, second_max_val - min_val)


# end_submission
