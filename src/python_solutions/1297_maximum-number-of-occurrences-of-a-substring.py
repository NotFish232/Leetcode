from common import *


# start_submission
class Solution:
    def maxFreq(self, s: str, maxLetters: int, minSize: int, maxSize: int) -> int:
        freqs = {}

        for i in range(len(s)):
            for k in range(minSize, maxSize + 1):
                if i + k - 1 < len(s):
                    sub = s[i : i + k]

                    uniques = len(set(sub))

                    if uniques <= maxLetters:
                        if sub not in freqs:
                            freqs[sub] = 0
                        freqs[sub] += 1

        return 0 if len(freqs) == 0 else max(freqs.values())


# end_submission
