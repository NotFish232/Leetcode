// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

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

        for _ in 0..len - n  - 1{
            if let Some(next_node) = ptr {
                ptr = &mut next_node.next;
            }
        }

        if let Some(ref mut node) = ptr {
            if let Some(next_node) = &node.next {
                node.next = next_node.next.clone();
            }
        }

        head
    }
}
