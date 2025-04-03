#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rest = None;

        while let Some(mut n) = head {
            head = n.next;
            n.next = rest;
            rest = Some(n);
        }

        rest
    }
}
// end_submission
