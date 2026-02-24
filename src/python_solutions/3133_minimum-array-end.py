from common import *


# start_submission
class Solution:
    def minEnd(self, n: int, x: int) -> int:
        n_bin = list(bin(n - 1)[2:])
        x_bin = list(bin(x)[2:])

        ans = deque()

        while n_bin or x_bin:
            if len(x_bin) == 0:
                ans.appendleft(n_bin.pop())
            elif len(n_bin) == 0:
                ans.appendleft(x_bin.pop())
            else:
                if x_bin[-1] == "0":
                    x_bin.pop()
                    ans.appendleft(n_bin.pop())
                else:
                    ans.appendleft(x_bin.pop())

        return int("".join(ans), base=2)


# end_submission
