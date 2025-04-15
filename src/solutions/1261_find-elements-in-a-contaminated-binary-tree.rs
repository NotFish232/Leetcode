#[allow(unused)]
use crate::stubs::*;

struct Solution;

// start_submission
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
        Self { hs }
    }

    fn find(&self, target: i32) -> bool {
        self.hs.contains(&target)
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, hs: &mut HashSet<i32>, val: i32) {
        if let Some(n) = node {
            hs.insert(val);

            Self::dfs(n.borrow().left.as_ref(), hs, val * 2 + 1);
            Self::dfs(n.borrow().right.as_ref(), hs, val * 2 + 2);
        }
    }
}
// end_submission
