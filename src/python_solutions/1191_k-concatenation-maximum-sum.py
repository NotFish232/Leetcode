from common import *


# start_submission
class Solution:
    def kConcatenationMaxSum(self, arr: List[int], k: int) -> int:
        n = len(arr)

        arr_sum = sum(arr)

        l_max_sum = 0
        l_sum = 0

        for i in range(n):
            l_sum += arr[i]
            l_max_sum = max(l_max_sum, l_sum)

        r_max_sum = 0
        r_sum = 0

        for i in reversed(range(n)):
            r_sum += arr[i]
            r_max_sum = max(r_max_sum, r_sum)

        im = 0
        max_im = 0

        for i in range(n):
            im = max(im + arr[i], arr[i])
            max_im = max(max_im, im)

        ans = max_im

        if k >= 2:
            ans = max(ans, l_max_sum + r_max_sum + max(0, (k - 2) * arr_sum))

        return ans % (10**9 + 7)


# end_submission
