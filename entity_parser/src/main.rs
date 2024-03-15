
pub fn entity_parser(text: String) -> String {
    let mut map = std::collections::hash_map::HashMap::new();
    map.insert("&quot;", "\"");
    map.insert("&apos;", "'");
    map.insert("&amp;", "&");
    map.insert("&gt;", ">");
    map.insert("&lt;", "<");
    map.insert("&frasl;", "/");
    let n = text.len();
    let mut pre_index = 0;
    let mut ans_utf8 = vec![];
    for index in 0..n - 3 {
        let current_value = text.get(index..index + 1).unwrap();
        if current_value != "&" {
            continue;
        }
        let two_char = text.get(index..index + 4).unwrap();
        if let Some(char) = map.get(two_char) {
            ans_utf8.push(text.get(pre_index..index).unwrap());
            ans_utf8.push(char);
            pre_index = index + 4;
            continue;
        }

        if let Some(three_char) = text.get(index..index + 5) {
            if let Some(char) = map.get(three_char) {
                ans_utf8.push(text.get(pre_index..index).unwrap());
                ans_utf8.push(char);
                pre_index = index + 5;
                continue;
            }
        }

        if let Some(four_char) = text.get(index..index + 6) {
            if let Some(char) = map.get(four_char) {
                ans_utf8.push(text.get(pre_index..index).unwrap());
                ans_utf8.push(char);
                pre_index = index + 6;
                continue;
            }
        }

        if let Some(five_char) = text.get(index..index + 7) {
            if let Some(char) = map.get(five_char) {
                ans_utf8.push(text.get(pre_index..index).unwrap());
                ans_utf8.push(char);
                pre_index = index + 7;
                continue;
            }
        }
    }
    ans_utf8.push(text.get(pre_index..n).unwrap());
    let ans: String = ans_utf8.iter().map(|item| item.to_owned()).collect();
    ans
}

#[test]
fn test1() {
    let text = "and I quote: &quot;...&quot;".to_owned();
    let ans = entity_parser(text);
    assert_eq!("and I quote: \"...\"", ans)
}

#[test]
fn test2() {
    let text = "&amp; is an HTML entity but &ambassador; is not.".to_owned();
    let ans = entity_parser(text);
    assert_eq!("& is an HTML entity but &ambassador; is not.", ans)
}

#[test]
fn test3() {
    let text = "&&gt;".to_owned();
    let ans = entity_parser(text);
    assert_eq!("&>", ans)
}
