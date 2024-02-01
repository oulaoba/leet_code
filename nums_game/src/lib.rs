// 小扣在秋日市集入口处发现了一个数字游戏。主办方共有 N 个计数器，计数器编号为 0 ~ N-1。每个计数器上分别显示了一个数字，小扣按计数器编号升序将所显示的数字记于数组 nums。每个计数器上有两个按钮，分别可以实现将显示数字加一或减一。小扣每一次操作可以选择一个计数器，按下加一或减一按钮。

// 主办方请小扣回答出一个长度为 N 的数组，第 i 个元素(0 <= i < N)表示将 0~i 号计数器 初始 所示数字操作成满足所有条件 nums[a]+1 == nums[a+1],(0 <= a < i) 的最小操作数。回答正确方可进入秋日市集。

// 由于答案可能很大，请将每个最小操作数对 1,000,000,007 取余。

pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
    fn insert(nums: &mut Vec<i32>, value: i32) -> usize {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] == value {
                l = mid;
                break;
            } else if nums[mid] > value {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        nums.insert(l, value);
        l
    }

    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let mut ans = vec![0; n];
    let (mut sum, mut lower, mut upper) = (nums[0] as i64, nums[0] as i64, 0);
    let mut sorted = Vec::with_capacity(n);
    sorted.push(nums[0]);
    for i in 1..n {
        let temp = nums[i] - i as i32;
        sum += temp as i64;
        let index = insert(&mut sorted, temp);
        let mid = sorted.len() / 2;
        if mid * 2 < sorted.len() {
            if index < mid {
                lower += temp as i64;
            } else {
                lower += sorted[mid] as i64;
            }
            upper = sum - lower;
            let ans_i = upper - lower + sorted[mid] as i64;
            ans[i] = (ans_i % MOD) as i32;
        } else {
            if index < mid {
                lower -= sorted[mid] as i64;
                lower += temp as i64;
            }
            upper = sum - lower;
            let ans_i = upper - lower;
            ans[i] = (ans_i % MOD) as i32;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let nums = vec![3, 4, 5, 1, 6, 7];
        let result = nums_game(nums);
        assert_eq!(result, vec![0, 0, 0, 5, 6, 7]);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = nums_game(nums);
        assert_eq!(result, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn it_works3() {
        let nums = vec![1, 1, 1, 2, 3, 4];
        let result = nums_game(nums);
        assert_eq!(result, vec![0, 1, 2, 3, 3, 3]);
    }
}
