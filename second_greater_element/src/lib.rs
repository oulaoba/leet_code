pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![-1; nums.len()];
    let mut smaller = Vec::new();
    let mut bigger = Vec::new();

    for (i, &v) in nums.iter().enumerate() {
        while !bigger.is_empty() && nums[*bigger.last().unwrap()] < v {
            ans[bigger.pop().unwrap()] = v;
        }
        let mut j = smaller.len();
        while j > 0 && nums[smaller[j - 1]] < v {
            j -= 1;
        }
        bigger.extend(smaller.drain(j..));
        smaller.push(i);
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 4, 0, 9, 6];
        let result = second_greater_element(nums);
        assert_eq!(result, vec![9, 6, 6, -1, -1]);
    }
}

pub fn is_palindrome(s: String) -> bool {
    // 大写字母 65～90，0～9 48～57
    let mut chars = s.chars().filter(|f| f.is_alphanumeric());

    while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
        if !a.eq_ignore_ascii_case(&b) {
            return false;
        }
    }
    return true;
    let chars: Vec<char> = s
        .to_uppercase()
        .chars()
        .filter(|f| {
            let c_num = *f as u8;
            (c_num > 47 && c_num < 58) || (c_num > 64 && c_num < 91)
        })
        .collect();
    let n = chars.len();
    for i in 0..n / 2 {
        if chars[i] != chars[n - 1 - i] {
            return false;
        }
    }

    true
}

#[test]
fn is_palindrome_works() {
    let s = String::from("A man, a plan, a canal: Panama");
    let ans = is_palindrome(s);
    assert!(ans);
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut t_chars: Vec<char> = t.chars().map(|m| m).collect();
    for s in s.chars().rev() {
        loop {
            let t = t_chars.pop();
            match t {
                Some(t) => {
                    if s == t {
                        break;
                    }
                }
                None => return false,
            }
        }
    }
    true
}

#[test]
fn it_works_subsequence() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    assert!(is_subsequence(s, t))
}

fn find_min_index(nums: Vec<i32>) -> usize {
    let mut left = 0;
    let max = nums[0];
    let mut right = nums.len();
    while left < right {
        let ans = left + (right - left) / 2;
        match (nums[ans] < nums[ans - 1], nums[ans] < max) {
            (false, true) => right = ans,
            (false, false) => left = ans + 1,
            _ => return ans,
        }
    }
    0
}

#[cfg(test)]
mod find_min_index_test {
    use super::*;
    #[test]
    fn find_min_index_test() {
        let nums = vec![4, 5, 1, 2, 3];
        let ans = find_min_index(nums);
        assert_eq!(2, ans)
    }

    #[test]
    fn find_min_index_test2() {
        let nums = vec![5, 1, 2, 3];
        let ans = find_min_index(nums);
        assert_eq!(1, ans)
    }

    #[test]
    fn find_min_index_test3() {
        let nums = vec![4, 1, 2, 3];
        let ans = find_min_index(nums);
        assert_eq!(1, ans)
    }

    #[test]
    fn find_min_index_test4() {
        let nums = vec![2, 3, 4, 1];
        let ans = find_min_index(nums);
        assert_eq!(3, ans)
    }
}
