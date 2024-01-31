pub fn minimum_total_price(
    n: i32,
    edges: Vec<Vec<i32>>,
    price: Vec<i32>,
    trips: Vec<Vec<i32>>,
) -> i32 {
    let n = n as usize;
    // 第一步，找到所有节点可以前往的节点
    let mut map = vec![vec![]; n];
    for edge in edges.iter() {
        let (from, to) = (edge[0] as usize, edge[1] as usize);
        map[from].push(to);
        map[to].push(from);
    }

    // 第二步，遍历所有的行程，将所有行程经过的节点的次数统计出来
    fn dfs(from: usize, fa: usize, cnt: &mut Vec<i32>, map: &Vec<Vec<usize>>, to: usize) -> bool {
        if from == to {
            cnt[from] += 1;
            return true;
        }

        for &y in &map[from] {
            if y != fa {
                if dfs(y, from, cnt, map, to) {
                    cnt[from] += 1;
                    return true;
                }
            }
        }
        false
    }

    let mut cnt = vec![0; n];
    for t in &trips {
        dfs(t[0] as usize, n, &mut cnt, &map, t[1] as usize);
    }

    // 第三步，计算所有经过的节点，打折或不打折的值
    fn dp(
        x: usize,
        fa: usize,
        price: &Vec<i32>,
        cnt: &Vec<i32>,
        map: &Vec<Vec<usize>>,
    ) -> (i32, i32) {
        let mut not_have = price[x] * cnt[x];
        let mut have = not_have / 2;
        for &y in &map[x] {
            if y != fa {
                let (nh, h) = dp(y, x, price, cnt, map);
                not_have += nh.min(h);
                have += nh;
            }
        }
        (not_have, have)
    }

    let (not_have, have) = dp(0, 0, &price, &cnt, &map);
    not_have.min(have)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];
        let price = vec![2, 2, 10, 6];
        let trips = vec![vec![0, 3], vec![2, 1], vec![2, 3]];
        let result = minimum_total_price(n, edges, price, trips);
        assert_eq!(result, 23);
    }
}
