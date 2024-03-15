
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans = 0;
    let mut set = std::collections::hash_set::HashSet::new();
    let n = s.len();
    for i in 0..n {
        let mut temp = 0;
        set.clear();
        for c in s[i..n].chars() {
            if set.insert(c) {
                temp += 1;
                ans = ans.max(temp);
            } else {
                break;
            }
        }
    }
    ans
}

pub fn fn2(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut ans = 0;

    for j in 0..s.len() {
        for x in i..j {
            if bytes[x] == bytes[j] {
                i = x + 1;
                break;
            }
            ans = ans.max(1 + j - i)
        }
    }
    ans as i32
}

#[test]
pub fn test() {
    let string = String::from("pwwkew");
    let ans = fn2(string);
    assert_eq!(ans, 3);
}

#[test]
pub fn test1() {
    let string = String::from("");

    let ans = fn2(string);

    assert_eq!(0, ans);
}

#[test]
pub fn test2() {
    let string = String::from(" ");

    let ans = fn2(string);

    assert_eq!(1, ans);
}

#[test]
pub fn test3() {
    let string = String::from("dvdf");

    let ans = fn2(string);

    assert_eq!(3, ans);
}

#[test]
pub fn test4() {
    let string = String::from("asjrgapa");

    let ans = length_of_longest_substring(string);

    assert_eq!(6, ans);
}
