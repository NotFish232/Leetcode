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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

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
