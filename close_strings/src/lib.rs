use std::borrow::BorrowMut;

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        false
    } else {
        let mut map1: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut map2: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        for c in word1.chars() {
            map1.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for c in word2.chars() {
            map2.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for kvp in map1.iter() {
            if let None = map2.get_key_value(kvp.0) {
                return false;
            }
        }
        let mut arr1: Vec<i32> = map1.values().clone().map(|v| *v).collect();
        let mut arr2: Vec<i32> = map2.values().clone().map(|v| *v).collect();
        arr1.sort();
        arr2.sort();
        for index in 0..arr1.len() {
            if arr1[index] != arr2[index] {
                return false;
            }
        }
        true
    }
}

pub fn close_strings2(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        false
    } else {
        let (mut arr1, mut arr2) = ([0; 26], [0; 26]);
        for c in word1.chars() {
            let i = c as usize - 97;
            arr1[i] += 1;
        }
        for c in word2.chars() {
            let i = c as usize - 97;
            arr2[i] += 1;
        }
        for i in 0..26 {
            if (arr1[i] == 0) != (arr2[i] == 0) {
                return false;
            }
        }
        arr1.sort();
        arr2.sort();
        arr1 == arr2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let word1 = String::from("");
        let word2 = String::from("");
        let result = close_strings(word1, word2);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works2() {
        let word1 = String::from("abc");
        let word2 = String::from("cba");
        let result = close_strings2(word1, word2);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works3() {
        let word1 = String::from("cabbba");
        let word2 = String::from("abbccc");
        let result = close_strings(word1, word2);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works4() {
        let word1 = String::from("cabbba");
        let word2 = String::from("aabbss");
        let result = close_strings(word1, word2);
        assert_eq!(result, false);
    }
    #[test]
    fn it_works5() {
        let word1 = String::from("aabbczz");
        let word2 = String::from("abbczzz");
        let result = close_strings(word1, word2);
        assert_eq!(result, false);
    }
}
