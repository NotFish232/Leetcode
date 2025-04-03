#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ptr = Some(Box::new(ListNode::new(0)));
        let mut cur = ptr.as_mut();
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut val = carry;
            if let Some(l1_node) = l1 {
                val += l1_node.val;
                l1 = l1_node.next;
            }
            if let Some(l2_node) = l2 {
                val += l2_node.val;
                l2 = l2_node.next;
            }
            carry = val / 10;
            val %= 10;

            if let Some(cur_node) = cur {
                cur_node.next = Some(Box::new(ListNode::new(val)));
                cur = cur_node.next.as_mut();
            }
        }
        ptr.unwrap().next
    }
}
// end_submission
