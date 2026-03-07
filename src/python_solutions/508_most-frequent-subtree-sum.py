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
    def solve(node, arr):
        if node is None:
            return 0

        val = node.val
        val += Solution.solve(node.left, arr)
        val += Solution.solve(node.right, arr)

        arr.append(val)

        return val

    def findFrequentTreeSum(self, root: Optional[TreeNode]) -> List[int]:
        d = defaultdict(int)

        res = []
        sums = Solution.solve(root, res)

        for x in res:
            d[x] += 1

        max_freq = max(d.values())

        return [k for k, v in d.items() if v == max_freq]


# end_submission
