from common import *


# start_submission
class Solution:
    def totalHammingDistance(self, nums: List[int]) -> int:
        d = {}

        for x in nums:
            for i in range(32):
                if i not in d:
                    d[i] = [0, 0]
                d[i][(x >> i) & 1] += 1

        return sum(x[0] * x[1] for x in d.values())


# end_submission
