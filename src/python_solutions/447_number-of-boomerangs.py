from common import *


# start_submission
class Solution:
    def numberOfBoomerangs(self, points: List[List[int]]) -> int:
        n = len(points)

        dists = [{} for _ in range(n)]

        for i in range(n):
            for j in range(i + 1, n):
                dist = (points[i][0] - points[j][0]) ** 2 + (
                    points[i][1] - points[j][1]
                ) ** 2
                if dist not in dists[i]:
                    dists[i][dist] = 0
                dists[i][dist] += 1

                if dist not in dists[j]:
                    dists[j][dist] = 0
                dists[j][dist] += 1

        ans = 0

        for d in dists:
            for v in d.values():
                ans += v * (v - 1)

        return ans


# end_submission
