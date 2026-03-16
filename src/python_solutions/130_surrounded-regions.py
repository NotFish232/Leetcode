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
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """

        n = len(board)
        m = len(board[0])

        dsu = DSU(n * m)

        dirs = [(-1, 0), (0, -1)]

        for i in range(n):
            for j in range(m):
                if board[i][j] == "O":
                    for d_i, d_j in dirs:
                        new_i = i + d_i
                        new_j = j + d_j

                        if 0 <= i < n and 0 <= j < m and board[new_i][new_j] == "O":
                            dsu.union(i * m + j, new_i * m + new_j)

        is_safe = {}

        for i in range(n):
            for j in range(m):
                if board[i][j] == "O":
                    k = dsu.find(i * m + j)

                    if i in [0, n - 1] or j in [0, m - 1]:
                        is_safe[k] = True
                    else:
                        if k not in is_safe:
                            is_safe[k] = False

        for i in range(n):
            for j in range(m):
                if board[i][j] == "O":
                    k = dsu.find(i * m + j)

                    if not is_safe[k]:
                        board[i][j] = "X"


# end_submission
