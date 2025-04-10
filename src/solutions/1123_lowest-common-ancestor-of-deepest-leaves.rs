#[allow(unused)]
use crate::stubs::*;

#[allow(dead_code)]
struct Solution;

// start_submission
use std::cell::RefCell;
use std::cmp::{Ordering, max};
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    fn count_depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }

        max(
            Self::count_depth(&node.as_ref().unwrap().borrow().left),
            Self::count_depth(&node.as_ref().unwrap().borrow().right),
        ) + 1
    }

    pub fn lca_deepest_leaves(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        loop {
            let left = root.as_ref().unwrap().borrow().left.clone();
            let right = root.as_ref().unwrap().borrow().right.clone();

            let l_depth = Self::count_depth(&left);
            let r_depth = Self::count_depth(&right);

            match l_depth.cmp(&r_depth) {
                Ordering::Less => root = right,
                Ordering::Greater => root = left,
                Ordering::Equal => break,
            };
        }

        root
    }
}
// end_submission
