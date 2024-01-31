pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut temp = Vec::from(nums1.get(0..m as usize).unwrap());
    let mut index = 0;
    loop {
        match (temp.is_empty(), nums2.is_empty()) {
            (true, true) => break,
            (false, true) => {
                nums1[index] = temp[0];
                temp.remove(0);
            }
            (true, false) => {
                nums1[index] = nums2[0];
                nums2.remove(0);
            }
            (false, false) => {
                if nums2[0] > temp[0] {
                    nums1[index] = temp[0];
                    temp.remove(0);
                } else {
                    nums1[index] = nums2[0];
                    nums2.remove(0);
                }
            }
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums1 = vec![1, 2, 3, 4, 0, 0, 0, 0];
        let mut nums2 = vec![2, 2, 4, 5];
        merge(&mut nums1, 4, &mut nums2, 4);
        let result = vec![1, 2, 2, 2, 3, 4, 4, 5];
        println!("{:?}", nums1);
        assert_eq!(result, nums1);
    }

    #[test]
    fn it_works2() {
        let mut nums1 = vec![0, 0, 0, 0];
        let mut nums2 = vec![2, 2, 4, 5];
        merge(&mut nums1, 0, &mut nums2, 4);
        let result = vec![2, 2, 4, 5];
        println!("{:?}", nums1);
        assert_eq!(result, nums1);
    }

    #[test]
    fn it_works3() {
        let mut nums1 = vec![2, 2, 4, 5];
        let mut nums2 = vec![];
        merge(&mut nums1, 4, &mut nums2, 0);
        let result = vec![2, 2, 4, 5];
        println!("{:?}", nums1);
        assert_eq!(result, nums1);
    }

    #[test]
    fn it_works4() {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        merge(&mut nums1, 0, &mut nums2, 0);
        let result: Vec<i32> = vec![];
        println!("{:?}", nums1);
        assert_eq!(result, nums1);
    }
}
