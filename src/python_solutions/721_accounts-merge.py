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
            if self.sizes[i_rep] < self.sizes[j_rep]:
                self.parents[i_rep] = j_rep
                self.sizes[j_rep] += self.sizes[i_rep]
            else:
                self.parents[j_rep] = i_rep
                self.sizes[i_rep] += self.sizes[j_rep]


class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        n = len(accounts)

        dsu = DSU(n)

        seen = {}

        for i in range(n):
            for j in range(1, len(accounts[i])):
                if accounts[i][j] in seen:
                    dsu.union(i, seen[accounts[i][j]])
                else:
                    seen[accounts[i][j]] = i

        res = defaultdict(set)

        for i in range(n):
            k = dsu.find(i)

            res[k].update(accounts[i][j] for j in range(1, len(accounts[i])))

        return [[accounts[k][0], *sorted(v)] for k, v in res.items()]


# end_submission
