#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut ptr: Option<&mut Box<ListNode>> = None;

        while list1.is_some() || list2.is_some() {
            let val;

            match (list1, list2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        val = n1.val;
                        list1 = n1.next;
                        list2 = Some(n2);
                    } else {
                        val = n2.val;
                        list1 = Some(n1);
                        list2 = n2.next;
                    }
                }
                (Some(n1), None) => {
                    val = n1.val;
                    list1 = n1.next;
                    list2 = None;
                }
                (None, Some(n2)) => {
                    val = n2.val;
                    list1 = None;
                    list2 = n2.next;
                }
                (None, None) => panic!(),
            };

            if let Some(p) = ptr {
                p.next = Some(Box::new(ListNode::new(val)));
                ptr = p.next.as_mut();
            } else {
                head = Some(Box::new(ListNode::new(val)));
                ptr = head.as_mut();
            }
        }

        head
    }
}
// end_submission
