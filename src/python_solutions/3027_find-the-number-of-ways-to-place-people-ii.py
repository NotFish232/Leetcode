from common import *


# start_submission
class Solution:
    def maximumPoints(self, enemyEnergies: List[int], currentEnergy: int) -> int:
        enemyEnergies = sorted(enemyEnergies)

        if currentEnergy < enemyEnergies[0]:
            return 0

        currentEnergy += sum(enemyEnergies[1:])

        return currentEnergy // enemyEnergies[0]


# end_submission
