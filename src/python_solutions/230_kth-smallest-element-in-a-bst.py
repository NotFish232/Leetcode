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
    def solve(node, count, k):
        if node is None:
            return count, -1

        left_count, left_ans = Solution.solve(node.left, count, k)

        if left_ans != -1:
            return 0, left_ans

        if left_count + 1 == k:
            return 0, node.val

        right_count, right_ans = Solution.solve(node.right, left_count + 1, k)

        if right_ans != -1:
            return 0, right_ans

        return right_count, -1

    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        return Solution.solve(root, 0, k)[1]


# end_submission
