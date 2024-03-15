pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = nums.iter().count();
    if n == 0 {
        return 0;
    }

    for index in 0..n - 1 {
        if (index - ans) % 2 == 0 {
            if nums[index] != nums[index + 1] {
            } else {
                ans += 1;
            }
        }
    }

    if (n - ans) % 2 != 0 {
        ans += 1;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::min_deletion;

    #[test]
    pub fn test1() {
        let data_in = vec![1, 1, 2, 3, 5];
        let ans = min_deletion(data_in);
        assert_eq!(1, ans);
    }

    #[test]
    pub fn test2() {
        let data_in = vec![];
        let ans = min_deletion(data_in);
        assert_eq!(0, ans);
    }

    #[test]
    pub fn test3() {
        let data_in = vec![1, 1, 1, 1, 1];
        let ans = min_deletion(data_in);
        assert_eq!(5, ans);
    }

    #[test]
    pub fn test4() {
        let data_in = vec![1, 1, 2, 1, 1];
        let ans = min_deletion(data_in);
        assert_eq!(3, ans);
    }
}
