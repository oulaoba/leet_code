pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let m = nums.len();
    let k = k as usize;
    let mut dp = vec![vec![0; k]; m + 1];
    for i in 0..k {
        dp[0][i] = nums[i];
    }
    for i in 1..m {
        for j in 0..k {
            todo!()
        }
    }
    0
}

pub fn max_result1(nums: Vec<i32>, k: i32) -> i32 {
    fn dfs(i: usize, nums: &Vec<i32>, k: usize, memo: &mut Vec<i32>) -> i32 {
        if i == 0 {
            return nums[0];
        }
        if memo[i] != i32::MIN {
            return memo[i];
        }
        let mut mx = i32::MIN;
        let start_index = i.saturating_sub(k);
        for j in start_index..i {
            mx = mx.max(dfs(j, nums, k, memo));
        }
        let ans = mx + nums[i];
        memo[i] = ans;
        ans
    }
    let mut memo = vec![i32::MIN; nums.len() + 1];
    dfs(nums.len() - 1, &nums, k as usize, &mut memo)
}

pub fn max_result2(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![0; n];
    f[0] = nums[0];
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    for i in 1..n {
        // 1. 出
        if *q.front().unwrap() + k < i {
            q.pop_front();
        }
        // 2. 转移
        f[i] = f[*q.front().unwrap()] + nums[i];
        // 3. 入
        while !q.is_empty() && f[i] >= f[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    f[n - 1]
}

pub fn max_result3(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    for i in 1..n {
        // 1. 出
        if *q.front().unwrap() + k < i {
            q.pop_front();
        }
        // 2. 转移
        nums[i] = nums[*q.front().unwrap()] + nums[i];
        // 3. 入
        while !q.is_empty() && nums[i] >= nums[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    nums[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = max_result2(vec![1, -1, -2, 4, -7, 3], 2);
        assert_eq!(result, 7);
    }
    #[test]
    fn it_works2() {
        let result = max_result(vec![10, -5, -2, 4, 0, 3], 3);
        assert_eq!(result, 17);
    }
    #[test]
    fn it_works3() {
        let result = max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2);
        assert_eq!(result, 0);
    }
}
