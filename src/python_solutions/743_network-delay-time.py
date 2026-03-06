from common import *


# start_submission
class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        edge_dict = defaultdict(lambda: [])

        for u, v, w in times:
            u -= 1
            v -= 1

            edge_dict[u].append((v, w))

        q = []
        dists = [float("inf")] * n

        q.append((0, k - 1))

        while len(q) > 0:
            dist, el = heappop(q)

            if dists[el] != float("inf"):
                continue

            dists[el] = dist

            for nbr, wght in edge_dict[el]:
                if dists[nbr] == float("inf"):
                    heappush(q, (dist + wght, nbr))

        ans = max(dists)

        return -1 if ans == float("inf") else ans


# end_submission
