#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut ptr = &mut head;

        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            let mut n1 = ptr.as_mut().unwrap().next.take();
            let n2 = n1.as_mut().unwrap().next.take();

            ptr.as_mut().unwrap().next = n2;
            n1.as_mut().unwrap().next = ptr.take();

            ptr.replace(n1.unwrap());

            ptr = &mut ptr.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }
}
// end_submission
