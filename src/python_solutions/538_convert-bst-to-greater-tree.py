from common import *


# start_submission
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    @staticmethod
    def solve(node, sum=0):
        if node is None:
            return sum

        right_sum = Solution.solve(node.right, sum)

        nodes_sum = node.val + right_sum

        node.val = nodes_sum

        left_sum = Solution.solve(node.left, nodes_sum)

        return left_sum

    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        Solution.solve(root)

        return root


# end_submission
