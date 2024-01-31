pub fn make_smallest_palindrome(s: String) -> String {
    let n = s.len();
    let mut ans = vec!['a'; n];
    let chars: Vec<char> = s.chars().collect();
    for front in 0..=n / 2 {
        let back = n - 1 - front;
        if chars[front] > chars[back] {
            ans[front] = chars[back];
        } else {
            ans[front] = chars[front]
        }
        ans[back] = ans[front];
    }
    ans.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("seven");
        let result = make_smallest_palindrome(s);
        assert_eq!(result, String::from("neven"));
    }

    #[test]
    fn it_works2() {
        let s = String::from("abba");
        let result = make_smallest_palindrome(s);
        assert_eq!(result, String::from("abba"));
    }

    #[test]
    fn it_works3() {
        let s = String::from("a");
        let result = make_smallest_palindrome(s);
        assert_eq!(result, String::from("a"));
    }
}
