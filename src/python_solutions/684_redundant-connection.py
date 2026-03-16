from common import *


# start_submission
class DSU:
    def __init__(self, n):
        self.parents = list(range(n))
        self.sizes = [0] * n

    def find(self, i):
        if i != self.parents[i]:
            self.parents[i] = self.find(self.parents[i])

        return self.parents[i]

    def union(self, i, j):
        i_rep = self.find(i)
        j_rep = self.find(j)

        if i_rep != j_rep:
            if self.sizes[i_rep] < self.sizes[j_rep]:
                self.parents[i_rep] = j_rep
                self.sizes[j_rep] += self.sizes[i_rep]
            else:
                self.parents[j_rep] = i_rep
                self.sizes[i_rep] += self.sizes[j_rep]


class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)

        dsu = DSU(n)

        for a, b in edges:
            if dsu.find(a - 1) == dsu.find(b - 1):
                return [a, b]

            dsu.union(a - 1, b - 1)


# end_submission
