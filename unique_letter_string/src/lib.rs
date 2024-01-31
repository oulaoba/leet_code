pub fn unique_letter_string(s: String) -> i32 {
    let mut map: std::collections::HashMap<char, Vec<i32>> = std::collections::HashMap::new();
    let mut index = 0;
    for c in s.chars() {
        if let Some(v) = map.get_mut(&c) {
            v.push(index);
        } else {
            map.insert(c, vec![-1, index]);
        }
        index += 1;
    }

    let mut ans = 0;
    for kvp in map.iter_mut() {
        let arr = kvp.1;
        arr.push(s.len() as i32);
        println!("{}:{:?}", kvp.0, arr);
        for index in 1..arr.len() - 1 {
            let temp = (arr[index] - arr[index - 1]) * (arr[index + 1] - arr[index]);
            ans += temp;
        }
    }

    ans as i32
}

#[test]
fn test1() {
    let s = String::from("LEETCODE");
    let ans = unique_letter_string(s);
    assert_eq!(92, ans);
}

#[test]
fn test2() {
    let s = String::from("ABC");
    let ans = unique_letter_string(s);
    assert_eq!(10, ans);
}

pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.repeat(n2 as usize).chars().collect();
    let total_len = s1.len() * n1 as usize;
    let (s1_len, s2_len) = (s1.len(), s2.len());
    let mut map = std::collections::HashMap::new();

    let mut loop_long = 0;
    let mut pre_loop_cnt = 0;
    let mut s2_index = 0;
    for i in 0..total_len {
        let s1_cur = i % s1_len;
        let s2_cur = s2_index % s2_len;
        if s1[s1_cur] == s2[s2_cur] {
            s2_index += 1;
            if s2_cur == 0 {
                if let Some((k, v)) = map.get_mut(&s1_cur) {
                    //出现相同坐标，证明出现循环
                    if *k == 2 {
                        // 已经转一圈了，可以计算了。
                        loop_long = i - *v;
                        break;
                    } else {
                        *k += 1;
                    }
                } else {
                    map.insert(s1_cur, (i, s2_index));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod get_max_repetitions_test {
    use super::get_max_repetitions;

    #[test]
    fn get_max_repetitions_test1() {
        let ans = get_max_repetitions("abc".to_owned(), 4, "ab".to_owned(), 2);
        assert_eq!(ans, 2);
    }

    #[test]
    fn get_max_repetitions_test2() {
        let ans = get_max_repetitions("abca".to_owned(), 2, "ab".to_owned(), 2);
        assert_eq!(ans, 1);
    }

    #[test]
    fn get_max_repetitions_test3() {
        let ans = get_max_repetitions("abc".to_owned(), 4, "ad".to_owned(), 2);
        assert_eq!(ans, 0);
    }

    #[test]
    fn get_max_repetitions_test4() {
        let ans = get_max_repetitions("baba".to_owned(), 11, "baab".to_owned(), 1);
        assert_eq!(ans, 7);
    }

    #[test]
    fn get_max_repetitions_test5() {
        let ans = get_max_repetitions("aba".to_owned(), 11, "a".to_owned(), 1);
        assert_eq!(ans, 22);
    }

    #[test]
    fn get_max_repetitions_test6() {
        let ans = get_max_repetitions("niconiconi".to_owned(), 99981, "nico".to_owned(), 81);
        assert_eq!(ans, 2468);
    }
}



