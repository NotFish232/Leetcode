from common import *


# start_submission
class Solution:
    def countCoveredBuildings(self, n: int, buildings: List[List[int]]) -> int:
        rows = {}
        cols = {}

        for a, b in buildings:
            if a not in rows:
                rows[a] = []
            rows[a].append(b)

            if b not in cols:
                cols[b] = []
            cols[b].append(a)

        for k in rows:
            rows[k] = sorted(rows[k])
        for k in cols:
            cols[k] = sorted(cols[k])

        cnt = 0

        for k, row in rows.items():
            for i in range(1, len(row) - 1):
                res = bisect_left(cols[row[i]], k)

                if res != 0 and res != len(cols[row[i]]) - 1:
                    cnt += 1

        return cnt


# end_submission
