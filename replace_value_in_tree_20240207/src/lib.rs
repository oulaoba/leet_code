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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = root;
    let mut left = 0;
    let mut right = 0;

    ans
}

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let n = preorder.len();
    let mut dic = std::collections::HashMap::with_capacity(n);
    for (i, &x) in inorder.iter().enumerate() {
        dic.insert(x, i);
    }

    fn dfs(
        preorder: &Vec<i32>,
        pre_l: usize,
        pre_r: usize,
        inorder: &Vec<i32>,
        in_l: usize,
        in_r: usize,
        dic: &std::collections::HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_l == pre_r {
            return None;
        }

        let left_size = dic[&preorder[pre_l]] - in_l;
        let left = dfs(
            preorder,
            pre_l + 1,
            pre_l + 1 + left_size,
            inorder,
            in_l + left_size,
            in_r,
            dic,
        );
        let right = dfs(
            preorder,
            pre_l + 1 + left_size,
            pre_r,
            inorder,
            in_l + 1 + left_size,
            in_r,
            dic,
        );
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[pre_l],
            left,
            right,
        })))
    }
    dfs(&preorder, 0, n, &inorder, 0, n, &dic)
}

pub fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    if preorder.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
    }

    let left_size = postorder.iter().position(|&x| x == preorder[1]).unwrap() + 1;
    let pre1 = preorder[1..1 + left_size].to_vec();
    let pre2 = preorder[1 + left_size..].to_vec();
    let post1 = postorder[..left_size].to_vec();
    let post2 = postorder[left_size..postorder.len() - 1].to_vec();

    let left = construct_from_pre_post(pre1, pre2);
    let right = construct_from_pre_post(post1, post2);
    return Some(Rc::new(RefCell::new(TreeNode {
        val: preorder[0],
        left,
        right,
    })));
    let n = preorder.len();
    let mut dic = HashMap::with_capacity(n);
    for (i, &v) in preorder.iter().enumerate() {
        dic.insert(v, i);
    }

    fn recursion() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })))
    }

    recursion()
}

pub fn construct_from_pre_post2(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    if preorder.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
    }

    let left_size = postorder.iter().position(|&x| x == preorder[1]).unwrap() + 1;
    let pre1 = preorder[1..1 + left_size].to_vec();
    let pre2 = preorder[1 + left_size..].to_vec();
    let post1 = postorder[..left_size].to_vec();
    let post2 = postorder[left_size..postorder.len() - 1].to_vec();

    let left = construct_from_pre_post(pre1, pre2);
    let right = construct_from_pre_post(post1, post2);
    return Some(Rc::new(RefCell::new(TreeNode {
        val: preorder[0],
        left,
        right,
    })));
}
pub fn construct_from_pre_post1(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        // 空节点
        return None;
    }
    if preorder.len() == 1 {
        // 叶子节点
        return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
    }
    let left_size = postorder.iter().position(|&x| x == preorder[1]).unwrap() + 1; // 左子树的大小
    let pre1 = preorder[1..1 + left_size].to_vec();
    let pre2 = preorder[1 + left_size..].to_vec();
    let post1 = postorder[..left_size].to_vec();
    let post2 = postorder[left_size..postorder.len() - 1].to_vec();
    let left = construct_from_pre_post1(pre1, post1);
    let right = construct_from_pre_post1(pre2, post2);
    Some(Rc::new(RefCell::new(TreeNode {
        val: preorder[0],
        left,
        right,
    })))
}

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    let mut dic = vec![];
    let mut current = vec![root.unwrap()];
    while !current.is_empty() {
        let mut temp = 0;
        let mut next = vec![];
        for node in current {
            let mut x = node.borrow_mut();
            temp += x.val as i64;
            if let Some(left) = x.left.take() {
                next.push(left);
            }
            if let Some(right) = x.right.take() {
                next.push(right);
            }
        }
        dic.push(temp);
        current = next;
    }
    if dic.len() < k as usize {
        -1
    } else {
        dic.sort_unstable();
        dic[dic.len() - k as usize]
    }
}
