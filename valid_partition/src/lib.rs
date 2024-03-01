// 2369. 检查数组是否存在有效划分
// 给你一个下标从 0 开始的整数数组 nums ，你必须将数组划分为一个或多个 连续 子数组。

// 如果获得的这些子数组中每个都能满足下述条件 之一 ，则可以称其为数组的一种 有效 划分：

// 子数组 恰 由 2 个相等元素组成，例如，子数组 [2,2] 。
// 子数组 恰 由 3 个相等元素组成，例如，子数组 [4,4,4] 。
// 子数组 恰 由 3 个连续递增元素组成，并且相邻元素之间的差值为 1 。例如，子数组 [3,4,5] ，但是子数组 [1,3,5] 不符合要求。
// 如果数组 至少 存在一种有效划分，返回 true ，否则，返回 false 。
/*
dp 思路，dp[] 存储是否作为分割点，
如何确定 dp[i] 是否可以作为分割点？
如果判断 2个相邻元素相等，则 nums[i-2] == nums[i-2] 并且 dp[i-1] == true,
若果判断 3个相邻元素相等，或3个元素为差值为1的递增数组。则判断 dp[i-3] == true 且 nums[i-3]，nums[i-2]，nums[i-1] 满足3个元素的条件。
dp[n] 就是答案
*/
pub fn valid_partition(nums: Vec<i32>) -> bool {
    fn check_two(x: i32, y: i32) -> bool {
        x == y
    }
    fn check_three(x: i32, y: i32, z: i32) -> bool {
        (x == y && x == z) || (x + 1 == y && y + 1 == z)
    }
    let mut f = vec![false; nums.len() + 1];
    f[0] = true;
    for i in 1..nums.len() {
        if (f[i - 1] && check_two(nums[i - 1], nums[i]))
            || (i > 1 && f[i - 2] && check_three(nums[i - 2], nums[i - 1], nums[i]))
        {
            f[i + 1] = true;
        }
    }
    f[nums.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = valid_partition(vec![4, 4, 4, 5, 6]);
        assert_eq!(result, true);
    }
}
