#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }

        let mut q = VecDeque::from([root.unwrap()]);
        let mut v = Vec::new();

        while !q.is_empty() {
            let node = Rc::clone(q.front().as_ref().unwrap());

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                v.push(node.borrow().val);
                q.pop_front();
            }

            if node.borrow().right.is_some() {
                q.push_front(node.borrow_mut().right.take().unwrap());
            }
            if node.borrow().left.is_some() {
                q.push_front(node.borrow_mut().left.take().unwrap());
            }
        }

        v
    }
}

// end_submission
