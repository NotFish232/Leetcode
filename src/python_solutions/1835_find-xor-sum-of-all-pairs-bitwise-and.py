from common import *


# start_submission
class Solution:
    def getXORSum(self, arr1: List[int], arr2: List[int]) -> int:
        n = 32

        bits1 = [0] * n
        bits2 = [0] * n

        for i in range(n):
            for x in arr1:
                bits1[i] += (x >> i) & 1

            for x in arr2:
                bits2[i] += (x >> i) & 1

        ans = 0

        for i in range(n):
            if bits1[i] * bits2[i] % 2 != 0:
                ans += 2**i

        return ans


# end_submission
