from common import *


# start_submission
class Solution:
    def gameOfLife(self, board: List[List[int]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        m = len(board)
        n = len(board[0])

        dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)]

        for i in range(m):
            for j in range(n):
                nbrs = defaultdict(int)

                for d_i, d_j in dirs:
                    new_i = i + d_i
                    new_j = j + d_j

                    if 0 <= new_i < m and 0 <= new_j < n:
                        nbrs[board[new_i][new_j] & 1] += 1

                if board[i][j] & 1 == 1:
                    if nbrs[1] < 2 or nbrs[1] > 3:
                        board[i][j] |= 2
                else:
                    if nbrs[1] == 3:
                        board[i][j] |= 2

        for i in range(m):
            for j in range(n):
                board[i][j] = (board[i][j] >> 1) ^ (board[i][j] & 1)


# end_submission
