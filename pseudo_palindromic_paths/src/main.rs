fn main() {
    println!("pseudo_palindromic_paths");
    let b_int = 0b_0001001;
    println!("b_int:{}", b_int);

    let mut set = std::collections::HashSet::new();
    println!("set.len():{}", set.len());
    println!("set.insert(0):{}", set.insert(0));
    println!("set.insert(0):{}", set.insert(0));
    println!("set.remove(&0):{}", set.remove(&0));

    println!("set.len():{}", set.len());
    println!("set.insert(1):{}", set.insert(1));
    println!("set.len():{}", set.len());
}

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

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // 解题思路，遍历二叉树，并且在遍历的过程中，检查当前路径是否为伪回文
    // 伪回文定义，路径经过的所有节点值的排列中，存在一个回文序列。
    // 简单理解，就是路径中所有出现的元素都是成对出现，或只有一个是出现单次的

    fn dfs(set: &mut std::collections::HashSet<i32>, node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = node {
            let x = x.borrow();
            if !set.insert(x.val) {
                set.remove(&x.val);
            }
            let res;
            if x.left.is_none() && x.right.is_none() {
                res = if set.len() <= 1 { 1 } else { 0 };
            } else {
                res = dfs(set, x.left.as_ref()) + dfs(set, x.right.as_ref());
            }
            if set.insert(x.val) {
                set.remove(&x.val);
            }
            return res;
        }
        0
    }

    let mut set = std::collections::HashSet::new();

    dfs(&mut set, root.as_ref())
}

#[test]
fn test1() {
    let mut root = TreeNode::new(2);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let root = Some(Rc::new(RefCell::new(root)));

    let ans = pseudo_palindromic_paths(root);
    assert_eq!(0, ans)
}
