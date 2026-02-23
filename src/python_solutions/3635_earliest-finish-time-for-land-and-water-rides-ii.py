from common import *


# start_submission
class Solution:
    def earliestFinishTime(
        self,
        landStartTime: List[int],
        landDuration: List[int],
        waterStartTime: List[int],
        waterDuration: List[int],
    ) -> int:
        ans = float("inf")

        for s1, d1, s2, d2 in [
            (landStartTime, landDuration, waterStartTime, waterDuration),
            (waterStartTime, waterDuration, landStartTime, landDuration),
        ]:
            s = min(t + d for t, d in zip(s1, d1))

            for t, d in zip(s2, d2):
                ans = min(ans, max(s, t) + d)

        return ans


# end_submission
