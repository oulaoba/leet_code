// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            dfs(node.right.as_ref(), s);
            *s += node.val;
            node.val = *s;
            dfs(node.left.as_ref(), s);
        }
    }
    let mut ans = 0;
    dfs(root.as_ref(), &mut ans);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
}
