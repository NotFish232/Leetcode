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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root == None {
            return Vec::new();
        }

        let mut q = VecDeque::from([root.unwrap()]);
        let mut v = Vec::new();

        while !q.is_empty() {
            let node = Rc::clone(q.front().as_ref().unwrap());
            
            v.push(node.borrow().val);
            q.pop_front();

            if node.borrow().right != None {
                q.push_front(node.borrow_mut().right.take().unwrap());
            }
            if node.borrow().left != None {
                q.push_front(node.borrow_mut().left.take().unwrap());
            }
        }

        v
    }
}
