from common import *


# start_submission
class Solution:
    def numTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        n = len(nums1)
        m = len(nums2)

        freq1 = {}
        freq2 = {}
        freq3 = {}
        freq4 = {}

        for i in range(n):
            x = nums1[i] ** 2

            if x not in freq1:
                freq1[x] = 0
            freq1[x] += 1

            for j in range(i + 1, n):
                x = nums1[i] * nums1[j]

                if x not in freq2:
                    freq2[x] = 0
                freq2[x] += 1

        for i in range(m):
            x = nums2[i] ** 2

            if x not in freq3:
                freq3[x] = 0
            freq3[x] += 1

            for j in range(i + 1, m):
                x = nums2[i] * nums2[j]

                if x not in freq4:
                    freq4[x] = 0
                freq4[x] += 1

        return sum(v * freq4.get(k, 0) for k, v in freq1.items()) + sum(
            v * freq3.get(k, 0) for k, v in freq2.items()
        )


# end_submission
