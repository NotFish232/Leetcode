from common import *


# start_submission
class Solution:
    def numSpecial(self, mat: List[List[int]]) -> int:
        m = len(mat)
        n = len(mat[0])

        r_cnt = [sum(mat[i][j] == 1 for j in range(n)) for i in range(m)]
        c_cnt = [sum(mat[i][j] == 1 for i in range(m)) for j in range(n)]

        cnt = 0

        for i in range(m):
            for j in range(n):
                if mat[i][j] == 1 and r_cnt[i] == 1 and c_cnt[j] == 1:
                    cnt += 1

        return cnt


# end_submission
