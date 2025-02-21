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
use std::collections::HashSet;
use std::rc::Rc;

struct FindElements {
    hs: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut hs = HashSet::new();
        Self::dfs(root.as_ref(), &mut hs, 0);
        FindElements { hs }
    }

    fn find(&self, target: i32) -> bool {
        self.hs.contains(&target)
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, hs: &mut HashSet<i32>, val: i32) {
        if let Some(n) = node {
            hs.insert(val);

            FindElements::dfs(n.borrow().left.as_ref(), hs, val * 2 + 1);
            FindElements::dfs(n.borrow().right.as_ref(), hs, val * 2 + 2);
        }
    }
}
