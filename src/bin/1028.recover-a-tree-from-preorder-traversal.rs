// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut traversal_chunks = VecDeque::new();
        let mut dash_count = 0;
        let mut s_num = String::new();
        for ch in traversal.chars().chain(['-']) {
            if ch == '-' {
                if !s_num.is_empty() {
                    traversal_chunks.push_back((s_num.parse().unwrap(), dash_count));
                    s_num = String::new();
                    dash_count = 0;
                }
                dash_count += 1;
            } else {
                s_num.push(ch);
            }
        }

        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(
            traversal_chunks.pop_front().unwrap().0,
        ))));
        let mut stack = vec![(Some(Rc::clone(root.as_ref().unwrap())), 0)];

        for (val, depth) in traversal_chunks {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));

            loop {
                if let Some((_, prev_depth)) = stack.last() {
                    if depth == prev_depth + 1 {
                        break;
                    }
                }

                stack.pop();
            }

            if let Some(prev) = stack.last() {
                if let Some(p) = &prev.0 {
                    if p.borrow().left == None {
                        p.borrow_mut().left = Some(Rc::clone(&node));
                    } else {
                        p.borrow_mut().right = Some(Rc::clone(&node));
                    }
                }

                stack.push((Some(node), prev.1 + 1));
            }
        }

        root
    }
}
