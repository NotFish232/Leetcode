from common import *


# start_submission
class Solution:
    def maximumSubarraySum(self, nums: List[int], k: int) -> int:
        n = len(nums)

        p_sum = [0] * (n + 1)

        for i in range(n):
            p_sum[i + 1] = p_sum[i] + nums[i]

        cnt = {}

        for i in range(k):
            if nums[i] not in cnt:
                cnt[nums[i]] = 0
            cnt[nums[i]] += 1

        ans = 0

        for i in range(len(nums) - k + 1):
            if len(cnt) == k:
                ans = max(ans, p_sum[i + k] - p_sum[i])

            if i + k < len(nums):
                if nums[i + k] not in cnt:
                    cnt[nums[i + k]] = 0
                cnt[nums[i + k]] += 1

            cnt[nums[i]] -= 1
            if cnt[nums[i]] == 0:
                del cnt[nums[i]]

        return ans


# end_submission
