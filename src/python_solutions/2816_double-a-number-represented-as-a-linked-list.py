from common import *


# start_submission
# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    @staticmethod
    def helper(node):
        if node is None:
            return 0

        carry = Solution.helper(node.next)

        node.val = 2 * node.val + carry

        carry = node.val // 10

        node.val %= 10

        return carry

    def doubleIt(self, head: Optional[ListNode]) -> Optional[ListNode]:
        carry = Solution.helper(head)

        if carry != 0:
            head = ListNode(carry, head)

        return head


# end_submission
