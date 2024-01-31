pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1; nums.len()];
    let mut right = vec![1; nums.len()];

    // 先从左边来算一遍
    for i in 1..nums.len() {
        ans[i] = nums[i - 1] * ans[i - 1]
    }
    // 再从右边算一遍
    for i in (0..nums.len() - 1).rev() {
        right[i] = nums[i + 1] * right[i + 1]
    }
    // 返回答案
    for i in 0..nums.len() {
        ans[i] *= right[i];
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4];
        let result = product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
}
