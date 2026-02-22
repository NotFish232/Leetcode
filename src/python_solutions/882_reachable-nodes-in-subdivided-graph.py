from common import *

# start_submission


class Solution:
    def reachableNodes(self, edges: List[List[int]], maxMoves: int, n: int) -> int:
        edge_dict = {}

        done = {}

        for a, b, t in edges:
            if a not in edge_dict:
                edge_dict[a] = []
            edge_dict[a].append(b)

            if b not in edge_dict:
                edge_dict[b] = []
            edge_dict[b].append(a)

            done[(a, b)] = (0, 0, t)

        q = []
        q.append((0, 0))

        seen = set()

        while len(q) > 0:
            dist, n = heappop(q)

            if n in seen:
                continue

            seen.add(n)

            for v in edge_dict.get(n, []):
                key = tuple(sorted([n, v]))

                l_c, r_c, t = done[key]

                if dist + t + 1 > maxMoves:
                    if n < v:
                        l_c = max(l_c, maxMoves - dist)
                    else:
                        r_c = max(r_c, maxMoves - dist)

                    done[key] = (l_c, r_c, t)

                else:
                    done[key] = (t, t, t)
                    heappush(q, (dist + t + 1, v))

        return len(seen) + sum(min(a + b, t) for a, b, t in done.values())


# end_submission
