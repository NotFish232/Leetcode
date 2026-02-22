# start_submission
class Solution:
    def maxBottlesDrunk(self, numBottles: int, numExchange: int) -> int:
        ans = 0

        while numBottles >= numExchange:
            ans += numExchange
            numBottles -= numExchange - 1
            numExchange += 1

        return ans + numBottles


# end_submission
