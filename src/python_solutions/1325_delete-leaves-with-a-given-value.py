from common import *


# start_submission
class Solution:
    @staticmethod
    def solve(node, tgt):
        if node is None:
            return None

        node.left = Solution.solve(node.left, tgt)
        node.right = Solution.solve(node.right, tgt)

        if node.val == tgt and node.left is None and node.right is None:
            return None

        return node

    def removeLeafNodes(
        self, root: Optional[TreeNode], target: int
    ) -> Optional[TreeNode]:
        return Solution.solve(root, target)


# end_submission
