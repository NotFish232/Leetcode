use std::ptr;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

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
