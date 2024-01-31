pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut ans = 1;
    let mut target = nums[0];
    for i in 1..nums.len() {
        if nums[i] != target {
            ans += 1;
            target = nums[i];
            nums[ans - 1] = target;
        }
    }

    ans as i32
}

pub fn remove_duplicates80(nums: &mut Vec<i32>) -> i32 {
    let mut ans = 1;
    let mut target = nums[0];
    let mut frequency = 1;
    for i in 1..nums.len() {
        if nums[i] != target {
            frequency = 1;
            ans += 1;
            target = nums[i];
            nums[ans - 1] = target;
        } else if frequency == 1 {
            frequency += 1;
            ans += 1;
            nums[ans - 1] = target;
        }
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 2, 2, 3, 3, 3, 4];
        let result = remove_duplicates(&mut nums);
        println!("{nums:?}");
        // assert_eq!(vec![1, 2, 3, 4], nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works1() {
        let mut nums = vec![1];
        let result = remove_duplicates(&mut nums);
        println!("{nums:?}");
        assert_eq!(vec![1], nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works2() {
        let mut nums = vec![1, 1, 2];
        let result = remove_duplicates(&mut nums);
        println!("{nums:?}");

        assert_eq!(result, 2);
    }

    #[test]
    fn it_works3() {
        let mut nums = vec![1, 1, 2];
        let result = remove_duplicates80(&mut nums);
        println!("{nums:?}");
        assert_eq!(vec![1, 1, 2], nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works4() {
        let mut nums = vec![1, 1, 1, 2];
        let result = remove_duplicates80(&mut nums);
        println!("{nums:?}");
        assert_eq!(vec![1, 1, 2, 2], nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works5() {
        let mut nums = vec![1, 1, 2, 2, 2,3];
        let result = remove_duplicates80(&mut nums);
        assert_eq!(result, 5);
        println!("{nums:?}");
        assert_eq!(vec![1, 1, 2, 2,3], nums);
    }
}
