from common import *


# start_submission
class Solution:
    def reconstructMatrix(
        self, upper: int, lower: int, colsum: List[int]
    ) -> List[List[int]]:
        two_counts = sum(x == 2 for x in colsum)
        if lower + upper != sum(colsum) or two_counts > lower or two_counts > upper:
            return []

        n = len(colsum)

        ans = [[0] * n for _ in range(2)]

        for i in range(n):
            if colsum[i] == 2:
                ans[0][i] = 1
                ans[1][i] = 1

                lower -= 1
                upper -= 1

                colsum[i] = 0

        for j in range(2):
            val = upper if j == 0 else lower

            for i in range(n):
                if val == 0:
                    break

                if colsum[i] > 0:
                    ans[j][i] = 1
                    colsum[i] -= 1
                    val -= 1

        return ans


# end_submission
