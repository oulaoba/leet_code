pub fn sol(nums: Vec<u8>) -> Vec<u8> {
    if nums.len() == 1 && nums[0] == 1 {
        return vec![1];
    }
    let cnt = nums.len();
    let mut ans = vec![0; cnt + 2];

    for i in 0..cnt {
        let sum = ans[i] + ans[i + 1] + ans[i + 2];
        if nums[i] > sum {
            ans[i + 2] = 1;
        } else if nums[i] < sum {
            ans[i - 1] = 1;
            ans[i] = 0;
        }
    }

    for i in (0..cnt).rev() {
        let sum = ans[i] + ans[i + 1] + ans[i + 2];
        if nums[i] > sum {
            ans[i] = 0;
        } else if nums[i] < sum {
            ans[i + 1] = 1;
        }
    }

    ans.remove(0);
    ans.remove(ans.len() - 1);
    ans
}

#[cfg(test)]
mod sol_test {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 2, 2, 1, 0];
        let ans = vec![0, 1, 1, 0, 1, 0];
        assert_eq!(sol(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 1];
        let ans = vec![0, 1, 0];
        assert_eq!(sol(nums), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 1, 1];
        let ans = vec![0, 1, 0, 0, 1];
        assert_eq!(sol(nums), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![2, 3, 3, 3, 2];
        let ans = vec![1, 1, 1, 1, 1];
        assert_eq!(sol(nums), ans);
    }
}
