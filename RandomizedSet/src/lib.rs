struct RandomizedSet {
    map: std::collections::HashMap<i32, usize>,
    data: [i32; 20000],
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use rand::Rng;
impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: [0; 20000],
            map: std::collections::HashMap::new(),
            count: 0,
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            // 所有新元素都放在最后面
            self.map.insert(val, self.count);
            self.data[self.count] = val;
            self.count += 1;
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            // 要移除的元素的当前下标
            let index = self.map.remove(&val).unwrap();
            // 删除最后一个元素，并把最后一个元素的值赋值到要删除的位置，并更新map中的下标
            self.count -= 1;
            if self.count != index {
                let last_num = self.data[self.count];
                self.map.insert(last_num, index);
                self.data[index] = last_num;
            }
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0, self.count);
        self.data[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_struct3() {
        let mut s = RandomizedSet::new();
        s.insert(100);
        s.insert(100);

        s.insert(1011);
        s.insert(1011);
        s.insert(1011);
        s.insert(1012);
        s.insert(1013);
        s.insert(1013);
        s.insert(1014);
        s.remove(0);
        s.remove(1013);
        s.remove(1014);
        s.remove(1012);
        s.remove(1011);
        s.remove(1011);
        s.remove(100);

        let random = s.get_random();
        let random = s.get_random();
        let random = s.get_random();
        let random = s.get_random();
        let random = s.get_random();
        let random = s.get_random();
    }

    #[test]
    fn it_works_struct2() {
        let mut s = RandomizedSet::new();
        s.remove(0);
        s.remove(0);
        s.insert(0);
        let random = s.get_random();
        s.remove(0);
        s.insert(0);
    }

    #[test]
    fn it_works_struct() {
        let mut s = RandomizedSet::new();
        s.insert(0);
        s.insert(1);
        s.remove(0);
        s.insert(2);
        s.remove(1);
        let random = s.get_random();
    }
}
