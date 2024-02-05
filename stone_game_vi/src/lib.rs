/*
Alice 和 Bob 轮流玩一个游戏，Alice 先手。

一堆石子里总共有 n 个石子，轮到某个玩家时，他可以 移出 一个石子并得到这个石子的价值。Alice 和 Bob 对石子价值有 不一样的的评判标准 。双方都知道对方的评判标准。

给你两个长度为 n 的整数数组 aliceValues 和 bobValues 。aliceValues[i] 和 bobValues[i] 分别表示 Alice 和 Bob 认为第 i 个石子的价值。

所有石子都被取完后，得分较高的人为胜者。如果两个玩家得分相同，那么为平局。两位玩家都会采用 最优策略 进行游戏。

请你推断游戏的结果，用如下的方式表示：

如果 Alice 赢，返回 1 。
如果 Bob 赢，返回 -1 。
如果游戏平局，返回 0 。
*/

pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut dic = Vec::with_capacity(alice_values.len());
    for (i, a) in alice_values.into_iter().enumerate() {
        dic.push((a, bob_values[i]))
    }
    dic.sort_by(|(fa, fb), (sa, sb)| (sa + sb).cmp(&(fa + fb)));
    let mut alice = 0;
    for (i, (a, b)) in dic.into_iter().enumerate() {
        if i % 2 == 0 {
            alice += a
        } else {
            alice -= b
        }
    }
    alice.signum()
}

/// Maybe on the wrong way
pub fn stone_game_vi1(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut alice_dic = vec![std::collections::HashSet::new(); 101];
    let mut bob_dic = vec![std::collections::HashSet::new(); 101];
    // 把两个人的分数从大到小排好
    for (i, &v) in alice_values.iter().enumerate() {
        alice_dic[v as usize].insert(i);
    }
    for (i, &v) in bob_values.iter().enumerate() {
        bob_dic[v as usize].insert(i);
    }

    // 如果己方最大的大于对方最大的，拿己方最大的，
    // 否则拿对方的最大分的石子，如果存在多个最大分相同的，选自己的最大分的。
    let (mut alice, mut bob) = (0, 0);
    let (mut alice_index, mut bob_index) = (100, 100);

    for i in 0..alice_values.len() {
        // 让两个指针都指向当前的最大值
        while alice_dic[alice_index].is_empty() {
            alice_index -= 1;
        }
        while bob_dic[bob_index].is_empty() {
            bob_index -= 1;
        }
        if i % 2 == 0 {
            // Alice 的回合
            if alice_index >= bob_index {
                // Alice 的值较大，所以拿自己的，并尽可能的减少 bob 的。
                alice += alice_index;
                let (mut remove, mut max) = (0, 0);
                for &a in alice_dic[alice_index].iter() {
                    if max < bob_values[a] {
                        max = bob_values[a];
                        remove = a;
                    }
                }
                alice_dic[alice_index].remove(&remove);
                bob_dic[max as usize].remove(&remove);
            } else {
                // bob 的值较大，所以干掉 bob 的，并尽可能让自己的最大
                let (mut remove, mut max) = (0, 0);
                for &a in bob_dic[bob_index].iter() {
                    if max < alice_values[a] {
                        max = alice_values[a];
                        remove = a;
                    }
                }

                alice += alice_values[remove] as usize;
                alice_dic[max as usize].remove(&remove);
                bob_dic[bob_index].remove(&remove);
            }
        } else {
            // Bob 的回合
            if bob_index >= alice_index {
                bob += bob_index;

                let (mut remove, mut max) = (0, 0);
                for &a in bob_dic[bob_index].iter() {
                    if max < alice_values[a] {
                        max = alice_values[a];
                        remove = a;
                    }
                }
                bob_dic[bob_index].remove(&remove);
                alice_dic[max as usize].remove(&remove);
            } else {
                let (mut remove, mut max) = (0, 0);
                for &a in alice_dic[alice_index].iter() {
                    if max < alice_values[a] {
                        max = alice_values[a];
                        remove = a;
                    }
                }
                bob += bob_values[remove] as usize;
                bob_dic[max as usize].remove(&remove);
                alice_dic[alice_index].remove(&remove);
            }
        }
    }
    alice.cmp(&bob) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let alice_values = vec![1, 3];
        let bob_values = vec![2, 1];
        let result = stone_game_vi(alice_values, bob_values);
        assert_eq!(result, 1)
    }

    #[test]
    fn test2() {
        let alice_values = vec![1, 2];
        let bob_values = vec![3, 1];
        let result = stone_game_vi(alice_values, bob_values);
        assert_eq!(result, 0)
    }

    #[test]
    fn test3() {
        let alice_values = vec![2, 4, 3];
        let bob_values = vec![1, 6, 7];
        let result = stone_game_vi(alice_values, bob_values);
        assert_eq!(result, -1)
    }

    #[test]
    fn test4() {
        let alice_values = vec![6, 7, 5, 6, 5, 6, 9, 3, 7, 3, 5, 6, 10, 3, 2, 7, 2, 5, 10, 2];
        let bob_values = vec![
            8, 2, 9, 10, 3, 2, 1, 1, 10, 6, 9, 1, 1, 4, 10, 3, 7, 9, 6, 2,
        ];
        let result = stone_game_vi(alice_values, bob_values);
        assert_eq!(result, 1)
    }
}

/*
1690. 石子游戏 VII
石子游戏中，爱丽丝和鲍勃轮流进行自己的回合，爱丽丝先开始 。

有 n 块石子排成一排。每个玩家的回合中，可以从行中 移除 最左边的石头或最右边的石头，并获得与该行中剩余石头值之 和 相等的得分。当没有石头可移除时，得分较高者获胜。

鲍勃发现他总是输掉游戏（可怜的鲍勃，他总是输），所以他决定尽力 减小得分的差值 。爱丽丝的目标是最大限度地 扩大得分的差值 。

给你一个整数数组 stones ，其中 stones[i] 表示 从左边开始 的第 i 个石头的值，如果爱丽丝和鲍勃都 发挥出最佳水平 ，请返回他们 得分的差值 。
*/

pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut dic = vec![0; n + 1];
    for i in 0..n {
        dic[i + 1] = dic[i] + stones[i];
    }

    let mut memo = vec![vec![0; n]; n];

    fn dfs(i: usize, j: usize, s: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == j {
            // 递归边界
            return 0;
        }
        if memo[i][j] != 0 {
            return memo[i][j];
        }
        let res1 = s[j + 1] - s[i + 1] - dfs(i + 1, j, s, memo);
        let res2 = s[j] - s[i] - dfs(i, j - 1, s, memo);
        let a = res1.max(res2);
        memo[i][j] = a;
        a
    }
    dfs(0, n - 1, &dic, &mut memo)
}

#[cfg(test)]
mod stone_game_vii_test {
    use super::*;

    #[test]
    fn it_works1() {
        let stones = vec![5, 3, 1, 4, 2];
        let ans = stone_game_vii(stones);
        assert_eq!(ans, 6);
    }
}
