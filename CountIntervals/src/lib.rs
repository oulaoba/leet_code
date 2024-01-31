struct CountIntervals {
    count: i32,
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    fn new() -> Self {
        Self {
            count: 0,
            map: std::collections::BTreeMap::new(),
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        for kvp in self.map.iter_mut() {
            match (left >= *kvp.0, right <= *kvp.1) {
                (true, true) => {
                    // 比左边大，比右边小
                    // 要添加的区间在当前区间内
                    return;
                }
                (false, false) => {
                    // 比左边小，比右边大
                    // 要添加的区间包含当前区间
                    self.count += right - *kvp.1 + 1;
                    self.count += kvp.0 - left + 1;
                }
                (true, false) => {
                    // 比左边大，比右边大
                    // 要添加的区间不确定是否在在当前区间
                    self.count -= right - *kvp.1 + 1;
                    break;
                }
                (false, true) => {
                    // 比左边小，比右边小
                    // 要添加的区间不确定是否在在当前区间
                    self.count += kvp.0 - left + 1;
                }
            }
        }
        self.map.insert(left, right);
        self.count += right - left;
    }

    fn count(&self) -> i32 {
        self.count
    }
}

/*
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 * 1 <= left <= right <= 109
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = CountIntervals::new();
        obj.add(0, 10);
        println!("{}", obj.count());
        obj.add(5, 11);
        println!("{}", obj.count());
        obj.add(13, 18);
        println!("{}", obj.count());
        obj.add(0, 1);
        println!("{}", obj.count());
        assert!(true)
    }
}
