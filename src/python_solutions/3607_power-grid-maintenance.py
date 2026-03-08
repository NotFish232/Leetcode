from common import *


# start_submission
class DSU:
    def __init__(self, n: int):
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
            if self.sizes[i_rep] < self.sizes[j_rep]:
                self.parents[j_rep] = self.parents[i_rep]
                self.sizes[i_rep] += self.sizes[j_rep]
            else:
                self.parents[i_rep] = self.parents[j_rep]
                self.sizes[j_rep] += self.sizes[i_rep]


class Solution:
    def processQueries(
        self, c: int, connections: List[List[int]], queries: List[List[int]]
    ) -> List[int]:
        dsu = DSU(c)

        for a, b in connections:
            dsu.union(a - 1, b - 1)

        groups = {}

        for i in range(c):
            grp = dsu.find(i)
            if grp not in groups:
                groups[grp] = SortedSet()
            groups[grp].add(i)

        ans = []

        for a, b in queries:
            b -= 1

            grp = dsu.find(b)
            if a == 1:
                if b in groups[grp]:
                    ans.append(b + 1)
                else:
                    ans.append(groups[grp][0] + 1 if groups[grp] else -1)
            else:
                if b in groups[grp]:
                    groups[grp].remove(b)

        return ans


# end_submission
