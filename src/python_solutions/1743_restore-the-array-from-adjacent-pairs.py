from common import *


# start_submission
class Solution:
    def restoreArray(self, adjacentPairs: List[List[int]]) -> List[int]:
        num_edges = {}
        edges = {}

        for a, b in adjacentPairs:
            if a not in num_edges:
                num_edges[a] = 0
            num_edges[a] += 1

            if b not in num_edges:
                num_edges[b] = 0
            num_edges[b] += 1

            if a not in edges:
                edges[a] = []
            edges[a].append(b)

            if b not in edges:
                edges[b] = []
            edges[b].append(a)

        prev = None
        cur = next(iter(k for k, v in num_edges.items() if v == 1))

        res = [cur]

        while len(res) != len(adjacentPairs) + 1:
            old_prev = prev
            prev = cur
            cur = edges[cur][0] if edges[cur][0] != old_prev else edges[cur][1]

            res.append(cur)

        return res


# end_submission
