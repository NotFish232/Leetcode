from common import *


# start_submission
class Solution:
    @staticmethod
    def topo(k, adj):
        in_degrees = [0] * k

        for a in adj.values():
            for x in a:
                in_degrees[x] += 1

        q = deque()
        ans = []

        for i in range(k):
            if in_degrees[i] == 0:
                q.append(i)
                ans.append(i)

        while q:
            x = q.popleft()

            if x in adj:
                for y in adj[x]:
                    in_degrees[y] -= 1

                    if in_degrees[y] == 0:
                        q.append(y)
                        ans.append(y)

        if sum(in_degrees) > 0:
            return None

        return ans

    def buildMatrix(
        self, k: int, rowConditions: List[List[int]], colConditions: List[List[int]]
    ) -> List[List[int]]:
        row_adj = {}
        col_adj = {}

        for a, b in rowConditions:
            a -= 1
            b -= 1

            if a not in row_adj:
                row_adj[a] = []
            row_adj[a].append(b)

        for a, b in colConditions:
            a -= 1
            b -= 1

            if a not in col_adj:
                col_adj[a] = []
            col_adj[a].append(b)

        row = Solution.topo(k, row_adj)
        col = Solution.topo(k, col_adj)

        if row is None or col is None:
            return []

        k_to_r = dict(zip(row, range(k)))
        k_to_c = dict(zip(col, range(k)))

        mat = [[0] * k for _ in range(k)]

        for i in range(k):
            mat[k_to_r[i]][k_to_c[i]] = i + 1

        return mat


# end_submission
