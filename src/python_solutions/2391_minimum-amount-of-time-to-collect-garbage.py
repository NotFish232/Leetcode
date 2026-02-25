from common import *


# start_submission
class Solution:
    def garbageCollection(self, garbage: List[str], travel: List[int]) -> int:
        lst = []

        for g in garbage:
            d = defaultdict(int)

            for x in g:
                d[x] += 1

            lst.append(d)

        p_sum = [0] * (len(garbage))
        for i in range(len(travel)):
            p_sum[i + 1] = p_sum[i] + travel[i]

        total = 0

        for t in "MGP":
            cnt = 0
            last = 0

            for i, l in enumerate(lst):
                cnt += l[t]

                if l[t] > 0:
                    last = i

            total += cnt + p_sum[last]

        return total


# end_submission
