use std::vec;

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let cnt = temperatures.len();
    let mut ans = vec![0; cnt];
    let mut s = vec![];

    for i in 0..cnt {
        while !s.is_empty() && temperatures[s[s.len() - 1]] < temperatures[i] {
            if let Some(last) = s.pop() {
                ans[last] = (i - last) as i32
            }
        }
        s.push(i)
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = daily_temperatures(temperatures);
        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
}

pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let cnt = nums.len();
    let mut suf = vec![0; cnt];
    let mut g = Vec::new();

    for i in (0..cnt).rev() {
        let j = g.partition_point(|&v| nums[i] > v);
        if j == g.len() {
            g.push(nums[i]);
        } else {
            g[j] = nums[i];
        }
        suf[i] = j + 1;
    }

    g.clear();
    for i in 0..cnt {
        let j = g.partition_point(|&v| v < nums[i]);
        if j == g.len() {
            g.push(nums[i]);
        } else {
            g[j] = nums[i];
        }
        let pre = j + 1;
        if pre >= 2 && suf[i] >= 2 {
            ans = ans.max(pre + suf[i] - 1);
        }
    }
    ans as i32
}

pub fn remove_duplicate_letters(s: String) -> String {
    let mut ans = vec![];

    let mut map = std::collections::HashMap::new();

    for (i, v) in s.chars().enumerate() {
        map.insert(v, i);
    }
    let mut single = false;
    let mut set = std::collections::HashSet::new();
    for (i, v) in s.chars().enumerate() {
        if let Some(&c) = map.get(&v) {
            if i == c {
                single = true;
            }
            if single {
                // 已经有元素到大最大坐标，后续没有了
                while !ans.is_empty() && ans[ans.len() - 1] < v {
                    ans.push(v)
                }
            } else {
                // 所有的元素后面都有
                set.insert(v);
            }
        }
    }
    ans.iter().collect()
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut arr = vec![true; nums.len() + 1];
    for num in nums {
        arr[num as usize] = false;
    }
    arr.into_iter().position(|flag| flag).unwrap() as i32
}
