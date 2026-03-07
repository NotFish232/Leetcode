from common import *


# start_submission
class DSU:
    def __init__(self, n: int) -> None:
        self.parents = list(range(n))
        self.size = [1] * n

    def find(self, i: int) -> int:
        if self.parents[i] != i:
            self.parents[i] = self.find(self.parents[i])

        return self.parents[i]

    def union(self, i: int, j: int):
        i_par = self.find(i)
        j_par = self.find(j)

        if i_par != j_par:
            if self.size[i_par] < self.size[j_par]:
                self.parents[i_par] = j_par
                self.size[j_par] += self.size[i_par]
            else:
                self.parents[j_par] = i_par
                self.size[i_par] += self.size[j_par]


class Solution:
    def pathExistenceQueries(
        self, n: int, nums: List[int], maxDiff: int, queries: List[List[int]]
    ) -> List[bool]:
        combined = sorted(zip(nums, range(n)))
        dsu = DSU(n)

        for i in range(1, n):
            if combined[i][0] - combined[i - 1][0] <= maxDiff:
                dsu.union(combined[i][1], combined[i - 1][1])

        ans = []

        for a, b in queries:
            ans.append(dsu.find(a) == dsu.find(b))

        return ans


# end_submission
