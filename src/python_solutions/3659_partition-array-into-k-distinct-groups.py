from common import *


# start_submission
class Solution:
    def partitionArray(self, nums: List[int], k: int) -> bool:
        n = len(nums)

        if n % k != 0:
            return False

        d = defaultdict(int)

        for x in nums:
            d[x] += 1

        return all(k * v <= n for v in d.values())


# end_submission
