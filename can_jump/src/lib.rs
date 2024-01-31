pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len() - 1;
    let mut index = 0;
    let mut max_len = 0;
    while index < n {
        max_len = max_len.max(nums[index] as usize + index);
        if max_len >= n {
            return true;
        }
        if max_len < index + 1 {
            return false;
        }
        index += 1;
    }

    max_len >= n
}
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut max_index = 0;
    let mut end = 0;
    for i in 0..nums.len() - 1 {
        max_index = max_index.max(nums[i] as usize + i);
        if i == end {
            end = max_index;
            ans += 1;
        }
    }
    ans
}

pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut sub_count = citations.len();
    let mut res = [0; 1001];
    for c in citations {
        res[c as usize] += 1;
    }
    for i in 0..res.len() {
        if sub_count < i {
            break;
        }
        sub_count -= res[i];
        ans = i;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_h_index1() {
        let result = h_index(vec![3, 0, 5, 6, 1, 5, 5]);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works() {
        let result = can_jump(vec![3, 2, 1, 0, 4]);
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_2() {
        let result = can_jump(vec![2, 3, 1, 1, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_3() {
        let result = can_jump(vec![0]);
        assert_eq!(result, true);
    }
    #[test]
    fn it_works_4() {
        let result = can_jump(vec![0, 2, 3]);
        assert_eq!(result, false);
    }
    #[test]
    fn it_works_5() {
        let result = can_jump(vec![2, 5, 0, 0]);
        assert_eq!(result, false);
    }
}
