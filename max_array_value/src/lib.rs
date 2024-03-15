pub fn max_array_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut sum = nums[n - 1] as i64;
    for i in (0..n - 1).rev() {
        let x = nums[i] as i64;
        sum = if x <= sum { sum + x } else { x }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_array_value(vec![2, 3, 7, 9, 3]);
        assert_eq!(result, 21);
    }
}
