from common import *


# start_submission
class Solution:
    def findThePrefixCommonArray(self, A: List[int], B: List[int]) -> List[int]:
        n = len(A)

        a = 0
        b = 0

        ans = []

        for i in range(n):
            a |= 1 << A[i]
            b |= 1 << B[i]

            ans.append((a & b).bit_count())

        return ans


# end_submission
