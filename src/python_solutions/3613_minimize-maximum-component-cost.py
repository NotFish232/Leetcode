from common import *


# start_submission
class DSU:
    def __init__(self, n):
        self.parents = list(range(n))
        self.sizes = [1] * n

    def find(self, i):
        if i != self.parents[i]:
            self.parents[i] = self.find(self.parents[i])

        return self.parents[i]

    def union(self, i, j):
        i_rep = self.find(i)
        j_rep = self.find(j)

        if i_rep != j_rep:
            if self.sizes[i_rep] > self.sizes[j_rep]:
                self.parents[j_rep] = i_rep
                self.sizes[i_rep] += self.sizes[j_rep]
            else:
                self.parents[i_rep] = j_rep
                self.sizes[j_rep] += self.sizes[i_rep]


class Solution:
    def minCost(self, n: int, edges: List[List[int]], k: int) -> int:
        l = 0
        r = 10**6
        ans = -1

        while l <= r:
            m = (l + r) // 2

            dsu = DSU(n)

            for a, b, w in edges:
                if w <= m:
                    dsu.union(a, b)

            comp = len(set(dsu.find(i) for i in range(n)))

            if comp <= k:
                ans = m
                r = m - 1
            else:
                l = m + 1

        return ans


# end_submission
