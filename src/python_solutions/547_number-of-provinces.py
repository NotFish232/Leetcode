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
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        n = len(isConnected)

        dsu = DSU(n)

        for i in range(n):
            for j in range(n):
                if isConnected[i][j]:
                    dsu.union(i, j)

        return len(set(dsu.find(i) for i in range(n)))


# end_submission
