from common import *


# start_submission
class Solution:
    def bagOfTokensScore(self, tokens: List[int], power: int) -> int:
        tokens.sort()

        max_score = 0
        score = 0

        l = 0
        r = len(tokens) - 1

        while l <= r:
            if power >= tokens[l]:
                power -= tokens[l]
                score += 1

                l += 1
            else:
                if score == 0:
                    break

                power += tokens[r]
                score -= 1

                r -= 1

            max_score = max(max_score, score)

        return max_score


# end_submission
