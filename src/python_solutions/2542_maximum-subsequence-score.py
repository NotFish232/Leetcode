from common import *


# start_submission
class Solution:
    def maxScore(self, nums1: List[int], nums2: List[int], k: int) -> int:
        n = len(nums1)

        idxs = sorted(range(n), key=lambda x: -nums2[x])

        h = []
        sm = 0

        ans = 0

        for i in idxs:
            sm += nums1[i]
            heappush(h, nums1[i])

            if len(h) < k:
                continue

            if len(h) > k:
                sm -= heappop(h)

            ans = max(ans, sm * nums2[i])

        return ans


# end_submission
