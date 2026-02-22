from common import *


# start_submission
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:

    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []

        q = deque()
        q.append((root, 0))

        out = []

        while len(q) > 0:
            n, d = q.popleft()

            if d >= len(out):
                out.append([])

            out[d].append(n.val)

            if n.left is not None:
                q.append((n.left, d + 1))
            if n.right is not None:
                q.append((n.right, d + 1))

        return out


# end_submission
