/*
LCP 30. 魔塔游戏
小扣当前位于魔塔游戏第一层，共有 N 个房间，编号为 0 ~ N-1。每个房间的补血道具/怪物对于血量影响记于数组 nums，其中正数表示道具补血数值，即血量增加对应数值；负数表示怪物造成伤害值，即血量减少对应数值；0 表示房间对血量无影响。

小扣初始血量为 1，且无上限。假定小扣原计划按房间编号升序访问所有房间补血/打怪，为保证血量始终为正值，小扣需对房间访问顺序进行调整，每次仅能将一个怪物房间（负数的房间）调整至访问顺序末尾。请返回小扣最少需要调整几次，才能顺利访问所有房间。若调整顺序也无法访问完全部房间，请返回 -1。
*/

pub fn magic_tower(nums: Vec<i32>) -> i32 {
    let mut skt = std::collections::BinaryHeap::new();
    let mut ans = 0;
    let mut current = 1;
    let mut sum = 1;
    for num in nums {
        sum += num as i64;
        if num > 0 {
            // 补血
            current += num as i64;
        } else {
            let abs = num.abs();
            // 记录打怪点
            skt.push(abs);
            // 打怪
            while current <= abs as i64 {
                if skt.is_empty() {
                    return -1;
                } else {
                    current += skt.pop().unwrap() as i64;
                    ans += 1;
                }
            }
            current -= abs as i64
        }
    }
    if sum < 1 {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150]);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works2() {
        let result = magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50]);
        assert_eq!(result, -1);
    }
}
