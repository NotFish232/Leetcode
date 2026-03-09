from common import *


# start_submission
class Solution:
    def canConvertString(self, s: str, t: str, k: int) -> bool:
        if len(s) != len(t):
            return False

        gaps = defaultdict(int)

        for a, b in zip(s, t):
            if a != b:
                gaps[(ord(b) - ord(a) + 26) % 26] += 1

        return all(v <= (k - x + 26) // 26 for x, v in gaps.items())


# end_submission
