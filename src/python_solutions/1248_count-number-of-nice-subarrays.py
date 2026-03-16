from common import *


# start_submission
class Solution:
    def numberOfSubarrays(self, nums: List[int], k: int) -> int:
        d = defaultdict(int)

        d[0] = 1

        c = 0
        ans = 0

        for x in nums:
            if x & 1 == 1:
                c += 1

            ans += d[c - k]

            d[c] += 1

        return ans


# end_submission
