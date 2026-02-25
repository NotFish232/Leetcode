from common import *


# start_submission
class Solution:
    def minOperations(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)

        end = nums2[-1]

        cnt = 0

        min_dist = float("inf")

        for i in range(n):
            cnt += abs(nums1[i] - nums2[i])

            if nums1[i] <= end <= nums2[i] or nums2[i] <= end <= nums1[i]:
                min_dist = 0
            else:
                min_dist = min(min_dist, min(abs(nums1[i] - end), abs(nums2[i] - end)))

        return cnt + min_dist + 1


# end_submission
