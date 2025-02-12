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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut ptr: Option<&mut Box<ListNode>> = None;

        while lists.iter().any(|o| o.is_some()) {
            let mut min_val = i32::MAX;
            let mut min_idx = 0;

            for (i, list) in lists.iter().enumerate() {
                if let Some(n) = list {
                    if n.val < min_val {
                        min_val = n.val;
                        min_idx = i;
                    }
                }
            }

            if let Some(n) = lists[min_idx].take() {
                lists[min_idx] = n.next;
            }

            if let Some(p) = ptr {
                p.next = Some(Box::new(ListNode::new(min_val)));
                ptr = p.next.as_mut();
            } else {
                head = Some(Box::new(ListNode::new(min_val)));
                ptr = head.as_mut();
            }
        }

        head
    }
}
