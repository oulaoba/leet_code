pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut dic = std::collections::BTreeMap::new();
    let mut ans = 0;
    for num in nums {
        if num == 1 {
            ans += 1;
        } else {
            dic.entry(num)
                .and_modify(|cnt| {
                    if *cnt < 2 {
                        *cnt += 1
                    }
                })
                .or_insert(1);
        }
    }

    while let Some((first_k, first_v)) = dic.pop_first() {
        // 从下往上算
        if first_v % 2 == 0 {
            // 当前值出现两次才需要向上继续找，否则最大值就是当前值了
            if let Some(upper) = first_k.checked_mul(first_k) {
                // 是有效值才往上走
                if let Some(kvp) = dic.get_mut(&upper) {
                    *kvp += first_v;
                }
            }
        }
        ans = ans.max(first_v);
    }
    if ans % 2 == 0 {
        ans - 1
    } else {
        ans
    }
}

#[test]
fn test() {
    let ans = maximum_length(vec![5, 4, 1, 2, 2]);
    assert_eq!(3, ans);
    let ans = maximum_length(vec![1, 3, 2, 4]);
    assert_eq!(1, ans);
    let ans = maximum_length(vec![1, 16, 4, 25, 121]);
    assert_eq!(1, ans);
    let ans = maximum_length(vec![2, 4, 2, 4, 16, 16, 32]);
    assert_eq!(5, ans);
    let ans = maximum_length(vec![2, 4, 2, 4, 16, 16, 256]);
    assert_eq!(7, ans);
    let ans = maximum_length(vec![15, 15, 225, 225, 50625, 50625]);
    assert_eq!(5, ans);
}

pub fn find_rotate_steps(ring: String, key: String) -> i32 {
    let m = ring.len();
    let n = key.len();
    let rings = ring.as_bytes();
    let keys = key.as_bytes();
    // a~z 97~122
    let mut dic = vec![vec![]; 26];
    for i in 0..m {
        // 记录 ring 中每个字符的下标
        dic[rings[i] as usize - 97].push(i);
    }
    // dp[i][j] 存的是什么 ？ 存的是 从ring[j] 到 key[i] 向前或向后到最少步数
    let mut dp = vec![vec![usize::MAX; m]; n];
    // 处理第一个位置
    for &i in dic[keys[0] as usize - 97].iter() {
        dp[0][i] = i.min(m - i);
    }

    for i in 1..n {
        // 从 keys[1] 开始处理
        for &current in dic[keys[i] as usize - 97].iter() {
            // 当前元素所处的下标
            for &prv in dic[keys[i - 1] as usize - 97].iter() {
                // 前一个元素所处的下标
                let step = if current > prv {
                    current - prv
                } else {
                    prv - current
                };
                let step = step.min(m - step);
                dp[i][current] = dp[i][current].min(dp[i - 1][prv] + step);
            }
        }
    }
    let mut ans = usize::MAX;
    for &num in dp[n - 1].iter() {
        ans = ans.min(num)
    }
    (ans + n) as i32
}

#[test]
fn steps_test() {
    let ring = "godding".to_owned();
    let key = "gd".to_owned();
    let ans = find_rotate_steps(ring, key);
    assert_eq!(ans, 4)
}

#[test]
fn steps_test2() {
    let ring = "godding".to_owned();
    let key = "godding".to_owned();
    let ans = find_rotate_steps(ring, key);
    assert_eq!(ans, 13)
}

pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
    let n = nums.len() - 1;
    let mut dic: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();
    let mut is_same = true;
    for (i, &v) in nums.iter().enumerate() {
        dic.entry(v)
            .and_modify(|cnt| cnt.push(i))
            .or_insert(vec![i]);
        if is_same && v != nums[0] {
            is_same = false;
        }
    }
    if is_same {
        return 0;
    }
    let mut ans = n;
    let mut dic = dic.into_iter();
    while let Some((_k, v)) = dic.next() {
        let mut temp_ans = usize::MAX;
        if v.len() == 1 {
            temp_ans = n;
        } else {
            let temp_n = v.len();
            // 从最后一个元素到第一个元素到距离
            let distance = n - v[temp_n - 1] + v[0];
            temp_ans = distance;
            for i in 1..v.len() {
                temp_ans = temp_ans.max(v[i] - 1 - v[i - 1])
            }
        }
        ans = temp_ans.min(ans)
    }
    if ans % 2 == 0 {
        (ans / 2) as i32
    } else {
        (ans / 2 + 1) as i32
    }
}

#[test]
fn minimum_seconds_test1() {
    let nums = vec![2, 1, 3, 3, 2];
    let ans = minimum_seconds(nums);
    assert_eq!(ans, 2)
}

#[test]
fn minimum_seconds_test2() {
    let nums = vec![5, 5, 5, 5];
    let ans = minimum_seconds(nums);
    assert_eq!(ans, 0)
}

#[test]
fn minimum_seconds_test3() {
    let nums = vec![8, 13, 3, 3];
    let ans = minimum_seconds(nums);
    assert_eq!(ans, 1)
}

#[test]
fn minimum_seconds_test4() {
    let nums = vec![19, 20, 7, 7, 20];
    let ans = minimum_seconds(nums);
    assert_eq!(ans, 1)
}
