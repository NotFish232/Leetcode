from common import *


# start_submission
class Solution:
    def maximumElementAfterDecrementingAndRearranging(self, arr: List[int]) -> int:
        arr = sorted(arr)

        arr[0] = 1

        for i in range(1, len(arr)):
            if arr[i] > arr[i - 1]:
                arr[i] = arr[i - 1] + 1

        return arr[-1]


# end_submission
