from common import *


# start_submission
class Solution:
    def findValidSplit(self, nums: List[int]) -> int:
        n = len(nums)
        N = max(nums) + 1

        seive = [True] * N
        seive[0] = False
        seive[1] = False

        for i in range(2, N):
            if seive[i]:
                for j in range(2 * i, N, i):
                    seive[j] = False

        primes = [i for i, b in enumerate(seive) if b]

        def factor(x):
            l = []

            for p in primes:
                if p * p > x:
                    break

                while x % p == 0:
                    l.append(p)
                    x //= p

            if x > 1:
                l.append(x)

            return l

        factors = [factor(x) for x in nums]

        post = {}

        for f in factors:
            for x in f:
                if x not in post:
                    post[x] = 0
                post[x] += 1

        pre = {}

        for i in range(len(nums) - 1):
            for f in factors[i]:
                if f not in pre:
                    pre[f] = 0
                pre[f] += 1

                post[f] -= 1
                if post[f] == 0:
                    del post[f]

            if all(k not in post for k in pre):
                return i

        return -1


# end_submission
