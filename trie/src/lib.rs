use std::ops::Index;

#[derive(Clone)]
struct Trie {
    data: Vec<Option<Box<Trie>>>,
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            data: vec![None; 26],
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = self;
        for c in word.chars() {
            let c = c as usize - 96;
            current = current.data[c].get_or_insert(Box::new(Self::new()));
        }
        current.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = self;
        for c in word.chars() {
            let c = c as usize - 96;
            match &current.data[c] {
                Some(node) => current = node.as_ref(),
                None => return false,
            }
        }
        current.is_word
    }

    pub fn starts_with(&self, word: String) -> bool {
        let mut current = self;
        for c in word.chars() {
            let c = c as usize - 96;
            match &current.data[c] {
                Some(node) => current = node.as_ref(),
                None => return false,
            }
        }
        true
    }
}

pub fn min_extra_char_trie(s: String, dictionary: Vec<String>) -> i32 {
    let mut tire = Trie::new();
    for word in dictionary {
        let word = word.chars().rev().collect();
        tire.insert(word);
    }

    let chars: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut ans = 0;
    let mut temp = vec![];
    for i in (0..n).rev() {
        temp.push(chars[i]);
        let matchings = tire.starts_with(temp.iter().collect());
        if matchings {
            if tire.search(temp.iter().collect()) {
                temp.clear();
            }
        } else {
            ans += 1;
            temp.clear();
        }
    }

    ans as i32
}

pub fn min_extra_char_array(s: String, dictionary: Vec<String>) -> i32 {
    let map: std::collections::HashSet<String> = dictionary.into_iter().collect();
    let n = s.len();
    let mut f = vec![0; n + 1];
    for i in 0..n {
        f[i + 1] = f[i] + 1;
        for j in 0..=i {
            if map.contains(s.get(j..i + 1).unwrap()) {
                f[i + 1] = f[j].min(f[i + 1]);
            }
        }
    }
    f[n] as i32
}

#[test]
fn test() {
    let s = "leetscode".to_owned();
    let dictionary = vec!["leet".to_owned(), "code".to_owned(), "leetcode".to_owned()];
    let ans = min_extra_char_array(s, dictionary);
    assert_eq!(1, ans)
}

#[test]
fn test2() {
    let s = "sayhelloworld".to_owned();
    let dictionary = vec!["hello".to_owned(), "world".to_owned()];
    let ans = min_extra_char_array(s, dictionary);
    assert_eq!(3, ans)
}

#[test]
fn minimum_time_test() {
    let nums = vec![1, 2, 3];
    let ans = minimum_time(nums.clone(), nums, 4);
    assert_eq!(3, ans)
}

#[test]
fn minimum_time_test2() {
    let nums1 = vec![4, 4, 9, 10];
    let nums2 = vec![4, 4, 1, 3];
    let ans = minimum_time(nums1, nums2, 16);
    assert_eq!(4, ans)
}

#[test]
fn minimum_time_test3() {
    let nums1 = vec![7, 9, 8, 5, 8, 3];
    let nums2 = vec![0, 1, 4, 2, 3, 1];
    let ans = minimum_time(nums1, nums2, 37);
    assert_eq!(4, ans)
}

pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
    let mut pairs = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
    pairs.sort_unstable_by(|&a, &b| a.1.cmp(&b.1));

    let n = pairs.len();
    let mut f = vec![0; n + 1];
    for (i, &(a, b)) in pairs.iter().enumerate() {
        for j in (1..=i + 1).rev() {
            f[j] = f[j].max(f[j - 1] + a + b * j as i32);
        }
    }

    let s1 = nums1.iter().sum::<i32>();
    let s2 = nums2.iter().sum::<i32>();
    for (t, &v) in f.iter().enumerate() {
        if s1 + s2 * t as i32 - v <= x {
            return t as i32;
        }
    }
    -1
}
