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

macro_rules! tree_new {
    ($val: expr) => {
        Some(Rc::new(RefCell::new(TreeNode::new($val))))
    };
}

macro_rules! tree_copy {
    ($node: expr) => {
        Some(Rc::clone($node.as_ref().unwrap()))
    };
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        mut preorder: Vec<i32>,
        mut postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        println!("preorder: {preorder:?}");
        println!("postorder: {postorder:?}");

        let root = tree_new!(preorder.remove(0));
        let mut stack = vec![tree_copy!(root)];

        for val in preorder {
            let node = tree_new!(val);
            let val_idx = postorder.iter().position(|&x| x == val).unwrap();

            while let Some(prev) = stack.last() {
                let prev_idx = postorder
                    .iter()
                    .position(|&x| x == prev.as_ref().unwrap().borrow().val)
                    .unwrap();

                if val_idx < prev_idx {
                    break;
                }

                stack.pop();
            }

            if let Some(Some(prev)) = stack.last() {
                if prev.borrow().left == None {
                    prev.borrow_mut().left = tree_copy!(node);
                } else {
                    prev.borrow_mut().right = tree_copy!(node);
                }
            }

            stack.push(node);
        }

        root
    }
}
