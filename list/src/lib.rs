#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;

    for element in vec.into_iter().rev() {
        let mut node = Box::new(ListNode::new(element));
        node.next = current;
        current = Some(node);
    }
    current
}

pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut head) => {
            if let Some(next) = remove_nodes(head.next.take()) {
                if head.val < next.val {
                    Some(next)
                } else {
                    head.next = Some(next);
                    Some(head)
                }
            } else {
                Some(head)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let head = vec![5, 2, 13, 3, 8];
        let head = to_list(head);
        let ans = remove_nodes(head);
        assert_eq!(ans, to_list(vec![13, 8]));
    }
}

pub fn delete_duplicates2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut val, mut cnt) = (1001, 0);
    let mut ans = vec![];
    while head.is_some() {
        let node = head.unwrap();
        if node.val == val {
            cnt += 1;
        } else {
            val = node.val;
            cnt = 1;
            if ans.is_empty() {
                ans.push(val);
            }
        }
        head = node.next;
    }
    to_list(ans)
}

pub fn to_list1(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = head.as_mut();

    for element in vec {
        let mut new_node = Box::new(ListNode::new(element));
    }
    head.unwrap().next.take()
}

pub fn to_list2(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for i in (0..vec.len()).rev() {
        let mut node = Box::new(ListNode::new(vec[i]));
        node.next = head;
        head = Some(node)
    }
    head
}

pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
    beans.sort();
    let cnt = beans.len();
    let mut sum = 0;
    let mut max = 0;
    for (i, &v) in beans.iter().enumerate() {
        sum += v as usize;
        max = max.max(v as usize * (cnt - i))
    }
    (sum - max) as i64
}

#[test]
fn minimum_test() {
    let beans = vec![4, 1, 6, 5];
    let ans = minimum_removal(beans);
    assert_eq!(ans, 4);
}
