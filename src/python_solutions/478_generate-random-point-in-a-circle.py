from common import *


# start_submission
class Solution:

    def __init__(self, radius: float, x_center: float, y_center: float):
        self.radius = radius
        self.x_center = x_center
        self.y_center = y_center

    def randPoint(self) -> List[float]:
        s_radius = self.radius * math.sqrt(random.random())
        s_theta = 2 * math.pi * random.random()

        x = self.x_center + s_radius * math.cos(s_theta)
        y = self.y_center + s_radius * math.sin(s_theta)

        return [x, y]


# Your Solution object will be instantiated and called as such:
# obj = Solution(radius, x_center, y_center)
# param_1 = obj.randPoint()


# Your Solution object will be instantiated and called as such:
# obj = Solution(radius, x_center, y_center)
# param_1 = obj.randPoint()
# end_submission
