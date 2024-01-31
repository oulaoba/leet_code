pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ans = 0;
    for index in 0..nums.len() {
        if nums[index] != val {
            nums[ans] = nums[index];
            ans += 1;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![2, 2, 4, 5];
        let result = remove_element(&mut nums, 2);
        assert_eq!(result, 2);
    }
}
