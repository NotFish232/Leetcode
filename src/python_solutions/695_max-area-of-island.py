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
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])

        dirs = [(-1, 0), (0, -1)]

        dsu = DSU(n * m)

        for i in range(n):
            for j in range(m):
                if grid[i][j] == 1:
                    for di, dj in dirs:
                        new_i = i + di
                        new_j = j + dj

                        if new_i >= 0 and new_j >= 0 and grid[new_i][new_j] == 1:
                            dsu.union(i * m + j, new_i * m + new_j)

        return max(
            (dsu.sizes[i] for i in range(n * m) if grid[i // m][i % m] == 1), default=0
        )


# end_submission
