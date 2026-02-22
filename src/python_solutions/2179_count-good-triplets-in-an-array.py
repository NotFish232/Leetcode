from common import *


# start_submission
class STree:
    def __init__(self, v):
        self.n = len(v)
        self.nodes = [0] * (2 * self.n)

        for i in range(self.n):
            self.nodes[self.n + i - 1] = v[i]

        for i in reversed(range(self.n - 1)):
            self.nodes[i] = self.nodes[2 * i + 1] + self.nodes[2 * i + 2]

    def update(self, i, v):
        ptr = self.n + i - 1

        self.nodes[ptr] = v

        while ptr > 0:
            ptr = (ptr - 1) // 2

            self.nodes[ptr] = self.nodes[2 * ptr + 1] + self.nodes[2 * ptr + 2]

    def query(self, i, j):
        if i > j:
            return 0

        l = self.n + i - 1
        r = self.n + j - 1

        ans = 0

        while True:
            if l % 2 == 0:
                ans += self.nodes[l]
                l += 1

            if r % 2 == 1:
                ans += self.nodes[r]
                r -= 1

            if l > r:
                break

            l = (l - 1) // 2
            r = (r - 1) // 2

        return ans


class Solution:
    def goodTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        N = len(nums1)

        n2idx = {}
        for i, n in enumerate(nums2):
            n2idx[n] = i

        nums = [n2idx[n] for n in nums1]

        l_tree = STree([0] * N)
        r_tree = STree([1] * N)

        ans = 0

        for i in range(N):
            ans += l_tree.query(0, nums[i] - 1) * r_tree.query(nums[i] + 1, N - 1)

            l_tree.update(nums[i], 1)
            r_tree.update(nums[i], 0)

        return ans


# end_submission
