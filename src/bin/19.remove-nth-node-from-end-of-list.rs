#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut p = &head;
        let mut len = 0;

        while let Some(node) = p {
            p = &node.next;
            len += 1;
        }

        let mut ptr = &mut head;

        if n + 1 > len {
            return head.unwrap().next;
        }

        for _ in 0..len - n - 1 {
            if let Some(next_node) = ptr {
                ptr = &mut next_node.next;
            }
        }

        if let Some(node) = ptr {
            if let Some(next_node) = node.next.take() {
                node.next = next_node.next.clone();
            }
        }

        head
    }
}
// end_submission
