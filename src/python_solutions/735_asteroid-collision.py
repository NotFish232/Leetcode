from common import *


# start_submission
class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        res = []

        i = 0

        while i < len(asteroids):
            a = asteroids[i]

            if a < 0 and len(res) > 0 and res[-1] > 0:
                if abs(res[-1]) == abs(a):
                    i += 1
                    res.pop()
                elif abs(res[-1]) < abs(a):
                    res.pop()
                else:
                    i += 1
            else:
                res.append(a)
                i += 1

        return res


# end_submission
