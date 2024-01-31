fn main() {
    println!("Hello, world!");
}

struct SmallestInfiniteSet {
    values: Vec<i32>,
    smallest: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        let values: Vec<i32> = vec![0; 2147483647];
        Self {
            values,
            smallest: 1,
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        self.values[self.smallest as usize] = -1;
        let ans = self.smallest;
        if self.smallest != i32::MAX {
            let mut index = self.smallest as usize;
            while self.values[index] == -1 {
                index += 1;
            }
            self.smallest = index as i32;
        }
        ans as i32
    }

    fn add_back(&mut self, num: i32) {
        self.values[num as usize] = 0;
        if num < self.smallest {
            self.smallest = num
        }
    }
}

/*
 Your SmallestInfiniteSet object will be instantiated and called as such:
 let obj = SmallestInfiniteSet::new();
 let ret_1: i32 = obj.pop_smallest();
 obj.add_back(num);
*/

#[test]
fn test2() {
    let mut orignl = SmallestInfiniteSet::new();

    let ans = orignl.add_back(2);
    let ans2 = orignl.pop_smallest();
    assert_eq!(1, ans2);

    let ans3 = orignl.pop_smallest();
    assert_eq!(2, ans3);

    let ans4 = orignl.pop_smallest();
    assert_eq!(3, ans4);

    let ans5 = orignl.add_back(1);

    let ans6 = orignl.pop_smallest();
    assert_eq!(1, ans6);

    let ans7 = orignl.pop_smallest();
    assert_eq!(4, ans7);

    let ans8 = orignl.pop_smallest();
    assert_eq!(5, ans8);

    let ans9 = orignl.pop_smallest();
    assert_eq!(6, ans9);
}

#[test]
fn test() {
    let mut orignl = SmallestInfiniteSet::new();

    let ans = orignl.add_back(2);
    let ans2 = orignl.pop_smallest();
    assert_eq!(1, ans2);

    let ans3 = orignl.pop_smallest();
    assert_eq!(2, ans3);

    let ans4 = orignl.pop_smallest();
    assert_eq!(3, ans4);

    let ans5 = orignl.add_back(1);

    let ans6 = orignl.pop_smallest();
    assert_eq!(1, ans6);

    let ans7 = orignl.pop_smallest();
    assert_eq!(4, ans7);

    let ans8 = orignl.pop_smallest();
    assert_eq!(5, ans8);
}
