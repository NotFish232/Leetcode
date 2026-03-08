from common import *


# start_submission
class Solution:
    def minimumOperationsToMakeKPeriodic(self, word: str, k: int) -> int:
        n = len(word)

        freqs = defaultdict(int)

        for i in range(0, n, k):
            freqs[word[i : i + k]] += 1

        return n // k - max(freqs.values())


# end_submission
