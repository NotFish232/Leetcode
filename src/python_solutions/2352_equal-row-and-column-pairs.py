from common import *


# start_submission
class Solution:
    def equalPairs(self, grid: List[List[int]]) -> int:
        n = len(grid)

        row_matches = [set(range(n)) for _ in range(n)]

        for i in range(n):
            col_values = {}
            for j in range(n):
                if grid[i][j] not in col_values:
                    col_values[grid[i][j]] = set()
                col_values[grid[i][j]].add(j)

            for j in range(n):
                if grid[j][i] in col_values:
                    row_matches[j] &= col_values[grid[j][i]]
                else:
                    row_matches[j].clear()

        return sum(len(r) for r in row_matches)


# end_submission
