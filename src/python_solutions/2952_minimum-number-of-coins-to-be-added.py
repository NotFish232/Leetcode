from common import *


# start_submission
class Solution:
    def minimumAddedCoins(self, coins: List[int], target: int) -> int:
        n = len(coins)
        coins = sorted(coins)

        coin_i = 0

        current = 0

        count = 0

        while current < target:
            if coin_i < n and coins[coin_i] <= current + 1:
                current += coins[coin_i]
                coin_i += 1
            else:
                current += current + 1
                count += 1

        return count


# end_submission
