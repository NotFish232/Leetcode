from common import *


# start_submission
class Solution:
    def minElements(self, nums: List[int], limit: int, goal: int) -> int:
        sm = sum(nums)

        return (abs(sm - goal) + limit - 1) // limit


# end_submission
