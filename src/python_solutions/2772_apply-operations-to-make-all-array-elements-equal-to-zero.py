from common import *


# start_submission
class Solution:
    def checkArray(self, nums: List[int], k: int) -> bool:
        q = deque()

        diff = 0

        for i, n in enumerate(nums):
            if len(q) > 0 and q[0][0] == i - k:
                diff -= q.popleft()[1]

            if diff > n:
                return False

            if diff < n:
                if i + k > len(nums):
                    return False

                q.append((i, n - diff))
                diff = n

        return True


# end_submission
