// 1976. 到达目的地的方案数
// 中等
// 相关标签
// 相关企业
// 提示
// 你在一个城市里，城市由 n 个路口组成，路口编号为 0 到 n - 1 ，某些路口之间有 双向 道路。输入保证你可以从任意路口出发到达其他任意路口，且任意两个路口之间最多有一条路。

// 给你一个整数 n 和二维整数数组 roads ，其中 roads[i] = [ui, vi, timei] 表示在路口 ui 和 vi 之间有一条需要花费 timei 时间才能通过的道路。你想知道花费 最少时间 从路口 0 出发到达路口 n - 1 的方案数。

// 请返回花费 最少时间 到达目的地的 路径数目 。由于答案可能很大，将结果对 109 + 7 取余 后返回。

    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let e97 = 1_000_000_007;
        let n = n as usize;
        let mut dic = vec![vec![i64::MAX / 2; n]; n];
        for r in roads {
            let (x, y, c) = (r[0] as usize, r[1] as usize, r[2] as i64);
            dic[x][y] = c;
            dic[y][x] = c;
        }
        let mut dis = vec![i64::MAX / 2; n];
        dis[0] = 0;
        let mut f = vec![0; n];
        f[0] = 1;
        let mut done = vec![false; n];
        loop {
            let mut x = n;
            for (i, &ok) in done.iter().enumerate() {
                if !ok && (x == n || dis[i] < dis[x]) {
                    x = i;
                }
            }
            if x == n - 1 {
                return f[n - 1];
            }
            done[x] = true;
            for (y, &d) in dic[x].iter().enumerate() {
                let new_dis = dis[x] + d;
                if new_dis < dis[y] {
                    dis[y] = new_dis;
                    f[y] = f[x];
                } else if new_dis == dis[y] {
                    f[y] = (f[y] + f[x]) % e97;
                }
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let roads = vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ];
        let result = count_paths(7, roads);
        assert_eq!(result, 4);
    }
}
