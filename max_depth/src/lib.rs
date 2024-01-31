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
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.as_ref().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.as_ref().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

pub fn max_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = 0;
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut count: i32, ans: &mut i32) {
        match node {
            Some(node) => {
                count += 1;
                *ans = *ans.max(&mut count);
                let node = node.borrow();
                dfs(node.left.as_ref(), count, ans);
                dfs(node.right.as_ref(), count, ans);
            }
            None => return,
        }
    }
    dfs(root.as_ref(), 0, &mut ans);
    ans
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            return match (root.left.as_ref(), root.right.as_ref()) {
                (None, None) => 1,
                _ => 1 + dfs(root.left.as_ref()).max(dfs(root.right.as_ref())),
            };
        }
        0
    }

    dfs(root.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let init = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let tree = to_tree(init);
        let ans = max_depth2(tree);
        assert_eq!(3, ans);
    }
}

pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let cnt = mines.len();
    let n = n as usize;
    if cnt == n * n {
        return 0;
    }
    let mut dic = vec![vec![1; n]; n];
    // 创建矩阵
    for mine in mines {
        let (i, j) = (mine[0] as usize, mine[1] as usize);
        dic[i][j] = 0;
    }
    let mut up = dic.clone();
    let mut down = dic.clone();
    let mut left = dic.clone();
    let mut right = dic.clone();
    for i in 1..n - 1 {
        // i = 列 j = 行
        for j in 1..n - 1 {
            if dic[i][j] == 1 {
                // 上左
                up[i][j] = up[i - 1][j] + 1;
                left[i][j] = left[i][j - 1] + 1;
            }
            let rev_i = n - 1 - i;
            let rev_j = n - 1 - j;
            if dic[rev_i][rev_j] == 1 {
                // 右下
                down[rev_i][rev_j] = down[rev_i + 1][rev_j] + 1;
                right[rev_i][rev_j] = right[rev_i][rev_j + 1] + 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans = ans.max(up[i][j].min(down[i][j].min(right[i][j].min(left[i][j]))));
        }
    }
    ans
}

#[test]
fn sign_test() {
    let mines = vec![vec![4, 2]];
    let ans = order_of_largest_plus_sign(5, mines);
    assert_eq!(ans, 2)
}
#[test]
fn sign_test1() {
    let mines = vec![
        vec![0, 0],
        vec![0, 3],
        vec![1, 1],
        vec![1, 4],
        vec![2, 3],
        vec![3, 0],
        vec![4, 2],
    ];
    let ans = order_of_largest_plus_sign(5, mines);
    assert_eq!(ans, 1)
}

#[test]
fn sign_test2() {
    let mines = vec![
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![0, 4],
        vec![1, 0],
        vec![1, 1],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 0],
        vec![2, 1],
        vec![2, 3],
        vec![2, 4],
        vec![3, 1],
        vec![3, 2],
        vec![3, 3],
        vec![3, 4],
        vec![4, 0],
        vec![4, 1],
        vec![4, 2],
        vec![4, 3],
        vec![4, 4],
    ];
    let ans = order_of_largest_plus_sign(5, mines);
    assert_eq!(ans, 1)
}
