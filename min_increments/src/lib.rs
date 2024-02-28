pub fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut ans = 0;
    for i in (1..n / 2).rev() {
        // 自底向上求
        ans += (cost[2 * i - 1] - cost[2 * i]).abs();
        cost[i] += cost[2 * i - 1].max(cost[2 * i]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_increments_test2() {
        let cost = vec![1, 5, 2, 2, 3, 3, 1];
        let ans = min_increments(7, cost);
        assert_eq!(6, ans)
    }

    #[test]
    fn min_increments_test1() {
        let cost = vec![
            764, 1460, 2664, 764, 2725, 4556, 5305, 8829, 5064, 5929, 7660, 6321, 4830, 7055, 3761,
        ];
        let ans = min_increments(15, cost);
        assert_eq!(15735, ans)
    }
}
