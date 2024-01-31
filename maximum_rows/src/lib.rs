use std::vec;

pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let bits_len = matrix[0].len();
    let tab: Vec<_> = matrix
        .into_iter()
        .map(|a| {
            a.into_iter()
                .enumerate()
                .fold(0, |x, (i, b)| (x | (b << i) as u32))
        })
        .collect();

    let mut ans = 0;
    for x in 0u32..1 << bits_len {
        if x.count_ones() != num_select as u32 {
            continue;
        }
        let cnt = tab.iter().fold(0, |cnt, &a| cnt + ((a & x) == a) as i32);
        ans = ans.max(cnt);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works5() {
        let matrix = vec![
            vec![1, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 1, 1, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 1],
        ];
        let result = maximum_rows(matrix, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works4() {
        let matrix = vec![vec![1, 0], vec![0, 1], vec![1, 1]];
        let result = maximum_rows(matrix, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works() {
        let matrix = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]];
        let result = maximum_rows(matrix, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works1() {
        let matrix = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        let result = maximum_rows(matrix, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works3() {
        let matrix = vec![vec![0, 0, 1], vec![1, 0, 0]];
        let result = maximum_rows(matrix, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let matrix = vec![vec![1], vec![0]];
        let result = maximum_rows(matrix, 1);
        assert_eq!(result, 2);
    }
}

pub fn can_see_persons_count1(heights: Vec<i32>) -> Vec<i32> {
    fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
        // 单调递减的数组
        // 找到比 target 大的首个元素
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            if target < nums[mid] {
                // target > mid ,则要查找的元素在 mid ～ r
                l = mid + 1;
            } else {
                // 要查找的元素在 l ~ mid
                r = mid - 1;
            }
        }
        (nums.len() - r) as i32
    }
    let n = heights.len();
    let mut ans = vec![0; n];
    let mut stack = vec![];
    for i in (0..n).rev() {
        if stack.is_empty() {
            // 栈里没有数据，把当前元素加进去
            stack.push(heights[i]);
        } else {
            // 栈里有数据，则当前单调递减的栈，长度就是该位置的答案
            if heights[i] > stack[0] {
                // 如果当前元素大于栈中的最后一个元素，（栈，先进后出），则栈的长度，就是答案，并且，需要将栈清空
                ans[i] = stack.len() as i32;
                stack.clear();
            } else {
                // 否则就要计算答案，并维护 stack 的单调性
                // 计算答案，stack.len() - stack 中第一个比 heights[i] 大的元素的下标就是答案
                // 所以我们二分查找 stack ，计算答案
                ans[i] = binary_search(&stack, heights[i]);
                // 维护单调性
                let mut cnt = stack.len().checked_sub(1);
                while let Some(current) = cnt {
                    // 如果当前元素小于 最后一个元素，则需要弹出元素
                    if stack[current] < heights[i] {
                        stack.pop();
                    } else {
                        break;
                    }
                    cnt = current.checked_sub(1);
                }
            }
            stack.push(heights[i]);
        }
    }
    ans
}

fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut ans = vec![0; n];
    let mut stack = vec![];
    for (i, &v) in heights.iter().enumerate().rev() {
        if !stack.is_empty() {
            // 原地计算
            let mut cnt = stack.len().checked_sub(1);
            while let Some(index) = cnt {
                if heights[i] > stack[index] {
                    // 如果当前元素比栈顶元素大，就需要弹出栈顶元素
                    ans[i] += 1;
                    stack.pop();
                } else {
                    // 栈里还有元素，并且这个元素是第一个比 heights[i] 大的元素
                    ans[i] += 1;
                    break;
                }
                cnt = index.checked_sub(1);
            }
        }
        stack.push(v);
    }
    ans
}

#[test]
fn test1() {
    let heights = vec![10, 6, 8, 5, 11, 9];
    let ans = can_see_persons_count(heights);
    assert_eq!(ans, vec![3, 1, 2, 1, 1, 0]);
}

#[test]
fn test2() {
    let heights = vec![5, 1, 2, 3, 10];
    let ans = can_see_persons_count(heights);
    assert_eq!(ans, vec![4, 1, 1, 1, 0]);
}

#[test]
fn test3() {
    let heights = vec![10, 2, 5, 4, 3, 8, 9];
    let ans = can_see_persons_count(heights);
    assert_eq!(ans, vec![4, 1, 2, 2, 1, 1, 0]);
}

pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let n = max_heights.len();
    let mut pav_stack = vec![];
    let mut max_pav = vec![0; n];
    let mut rev_stack = vec![];
    let mut max_rev = vec![0; n];
    for i in (0..n).rev() {
        if rev_stack.is_empty() {
            rev_stack.push(max_heights[i] as i64);
            max_rev[i] = max_heights[i] as i64;
        } else {
            let mut index = rev_stack.len().checked_sub(1);
            let mut re_append = vec![];
            let mut rev_sum = max_rev[i + 1];
            while let Some(c_index) = index {
                let rev_top = rev_stack[c_index];
                if rev_top <= max_heights[i] as i64 {
                    break;
                } else {
                    let top = rev_stack.pop().unwrap();
                    rev_sum -= top;
                    re_append.insert(0, max_heights[i] as i64);
                }
                index = c_index.checked_sub(1);
            }
            re_append.insert(0, max_heights[i] as i64);
            for re in re_append {
                rev_stack.push(re);
                rev_sum += re;
            }
            max_rev[i] = rev_sum;
        }
    }
    for i in 0..n {
        if pav_stack.is_empty() {
            pav_stack.push(max_heights[i] as i64);
            max_pav[i] = max_heights[i] as i64;
        } else {
            let mut index = pav_stack.len().checked_sub(1);
            let mut re_append = vec![];
            let mut pav_sum = max_pav[i - 1];
            while let Some(c_index) = index {
                let pav_top = pav_stack[c_index];
                if pav_top <= max_heights[i] as i64 {
                    break;
                } else {
                    let top = pav_stack.pop().unwrap();
                    pav_sum -= top;
                    re_append.push(max_heights[i] as i64);
                }
                index = c_index.checked_sub(1);
            }
            re_append.push(max_heights[i] as i64);
            for re in re_append {
                pav_stack.push(re);
                pav_sum += re;
            }
            max_pav[i] = pav_sum;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(max_pav[i] + max_rev[i] - max_heights[i] as i64)
    }
    ans
}

#[test]
fn maximum_sum_test() {
    let max_heights = vec![5, 3, 4, 1, 1];
    let ans = maximum_sum_of_heights(max_heights);
    assert_eq!(ans, 13);
}
#[test]
fn maximum_sum_test2() {
    let max_heights = vec![6, 5, 3, 9, 2, 7];
    let ans = maximum_sum_of_heights(max_heights);
    assert_eq!(ans, 22);
}
