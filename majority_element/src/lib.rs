pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut ans = nums[0];
    let mut cnt = 1;
    for i in 0..nums.len() {
        map.entry(nums[i])
            .and_modify(|counter| {
                *counter += 1;
                if cnt < *counter {
                    ans = nums[i];
                }
                cnt = cnt.max(*counter);
            })
            .or_insert(1);
    }
    ans
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = (k as usize) % n;
    let temp = nums[0..n - k].to_vec();

    for i in 0..k {
        nums[i] = nums[n - k + i];
    }
    for i in 0..temp.len() {
        nums[i + k] = temp[i];
    }
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut min = prices[0];
    for price in prices {
        min = min.min(price);
        ans = ans.max(price - min);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_it_works() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let ans = vec![5, 6, 7, 1, 2, 3, 4];
        rotate(&mut nums, 3);
        assert_eq!(nums, ans)
    }
    #[test]
    fn it_works() {
        let nums = vec![1, 2, 2, 3];
        let result = majority_element(nums);
        assert_eq!(result, 2);
    }
}
