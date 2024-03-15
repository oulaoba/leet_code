
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        let total_count = nums1.len() + nums2.len();

        let mut remove_count = total_count / 2;
        if total_count % 2 == 0 {
            remove_count -= 1;
        }

        while remove_count > 0 {
            match (nums1.is_empty(), nums2.is_empty()) {
                (true, false) => {
                    nums2.remove(0);
                    remove_count -= 1;
                }
                (false, true) => {
                    nums1.remove(0);
                    remove_count -= 1;
                }
                (false, false) => {
                    let num1 = nums1[0];
                    let num2 = nums2[0];
                    if num1 < num2 {
                        nums1.remove(0);
                    } else {
                        nums2.remove(0);
                    }
                    remove_count -= 1;
                }
                (true, true) => (),
            }
        }
        let mut arr = vec![];
        for _ in 0..2 {
            match (nums1.is_empty(), nums2.is_empty()) {
                (false, false) => {
                    let n1 = nums1[0];
                    let n2 = nums2[0];
                    if n1 < n2 {
                        nums1.remove(0);
                        arr.push(n1);
                    } else {
                        nums2.remove(0);
                        arr.push(n2);
                    }
                }
                (false, true) => {
                    arr.push(nums1[0]);
                    nums1.remove(0);
                }
                (true, false) => {
                    arr.push(nums2[0]);
                    nums2.remove(0);
                }
                (true, true) => arr.push(arr[0]),
            }
        }

        if total_count % 2 == 0 {
            let temp = arr[0] as f64 + arr[1] as f64;
            temp / 2.0
        } else {
            return arr[0] as f64;
        }
    }

#[test]
fn test1() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.0, ans);
}

#[test]
fn test2() {
    let nums1 = vec![1, 3];
    let nums2 = vec![];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.0, ans);
}

#[test]
fn test3() {
    let nums1 = vec![1, 3];
    let nums2 = vec![1, 3];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.0, ans);
}

#[test]
fn test4() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![4, 5];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(3.0, ans);
}

#[test]
fn test5() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![5, 8, 10, 20];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(5.0, ans);
}

#[test]
fn test6() {
    let nums1 = vec![];
    let nums2 = vec![1, 2, 3, 4];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(2.5, ans);
}

#[test]
fn test7() {
    let nums1 = vec![1, 2];
    let nums2 = vec![-1, 3];
    let ans = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(1.5, ans);
}
