from common import *


# start_submission
# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    @staticmethod
    def solve(node, cur_max):
        if node is None:
            return 0

        count = 0

        if node.val >= cur_max:
            count += 1

        cur_max = max(cur_max, node.val)

        count += Solution.solve(node.left, cur_max) + Solution.solve(
            node.right, cur_max
        )

        return count

    def goodNodes(self, root: TreeNode) -> int:
        return Solution.solve(root, float("-inf"))


# end_submission
