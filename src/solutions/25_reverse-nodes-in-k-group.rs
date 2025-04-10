#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;
        for _ in 0..k {
            if let Some(n) = ptr {
                ptr = &mut n.next;
            } else {
                return head;
            }
        }

        let mut rest_of_list = Solution::reverse_k_group(ptr.take(), k);
        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next;
            node.next = rest_of_list;
            rest_of_list = Some(node);
        }

        rest_of_list
    }
}
// end_submission
