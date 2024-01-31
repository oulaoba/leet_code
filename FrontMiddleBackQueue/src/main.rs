fn main() {
    println!("FrontMiddleBackQueue");
}

use std::collections::VecDeque;
struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/

impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }
    fn balance(&mut self) {
        if self.left.len() < self.right.len() {
            self.left.push_back(self.right.pop_front().unwrap())
        } else if self.left.len() > self.right.len() + 1 {
            self.right.push_front(self.left.pop_back().unwrap())
        }
    }

    pub fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.balance();
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap())
        }
        self.left.push_back(val);
        self.balance();
    }

    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.balance();
    }

    fn pop_front(&mut self) -> i32 {
        let ans = match self.left.pop_front() {
            Some(val) => val,
            None => -1,
        };
        self.balance();
        ans
    }

    fn pop_middle(&mut self) -> i32 {
        let ans = match self.left.pop_back() {
            Some(val) => val,
            None => -1,
        };
        self.balance();
        ans
    }

    fn pop_back(&mut self) -> i32 {
        let ans = match self.right.pop_back() {
            Some(val) => val,
            None => match self.left.pop_back() {
                Some(val) => val,
                None => -1,
            },
        };
        self.balance();
        ans
    }
}

impl std::fmt::Display for FrontMiddleBackQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}<|>{:?}", self.left, self.right)
    }
}
/*
 Your FrontMiddleBackQueue object will be instantiated and called as such:
 let obj = FrontMiddleBackQueue::new();
 obj.push_front(val);
 obj.push_middle(val);
 obj.push_back(val);
 let ret_4: i32 = obj.pop_front();
 let ret_5: i32 = obj.pop_middle();
 let ret_6: i32 = obj.pop_back();
*/

#[test]
fn test() {
    let mut que = FrontMiddleBackQueue::new();
    let t1 = que.push_front(1);
    println!("t1:{:?}", que.to_string());
    let t2 = que.push_back(2);
    println!("t2:{:?}", que.to_string());
    let t3 = que.push_middle(3);
    println!("t3:{:?}", que.to_string());
    let t4 = que.push_middle(4);
    println!("t4:{:?}", que.to_string());

    let t5 = que.pop_front();
    println!("t5:{}&{:?}", t5, que.to_string());
    let t6 = que.pop_middle();
    println!("t6:{}&{:?}", t6, que.to_string());
    let t7 = que.pop_middle();
    println!("t7:{}&{:?}", t7, que.to_string());
    let t8 = que.pop_back();
    println!("t8:{}&{:?}", t8, que.to_string());
    let t9 = que.pop_front();
    println!("t9:{}&{:?}", t9, que.to_string());
}
