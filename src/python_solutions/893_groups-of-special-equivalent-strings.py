from common import *


# start_submission
class Solution:
    def numSpecialEquivGroups(self, words: List[str]) -> int:
        s = set()

        for w in words:
            e = "".join(sorted(w[i] for i in range(0, len(w), 2)))
            o = "".join(sorted(w[i] for i in range(1, len(w), 2)))

            s.add(e + o)

        return len(s)


# end_submission
