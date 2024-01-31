pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut g = vec![vec![]; roads.len() + 1];
    for e in &roads {
        let x = e[0] as usize;
        let y = e[1] as usize;
        g[x].push(y); // 记录每个点的邻居
        g[y].push(x);
    }
    let mut ans = 0i64;
    dfs(0, 0, &g, seats, &mut ans);
    ans
}

fn dfs(x: usize, fa: usize, g: &Vec<Vec<usize>>, seats: i32, ans: &mut i64) -> i32 {
    let mut size = 1;
    for &y in &g[x] {
        if y != fa {
            // 递归子节点，不能递归父节点
            size += dfs(y, x, g, seats, ans); // 统计子树大小
        }
    }
    if x != 0 {
        // x 不是根节点
        *ans += ((size - 1) / seats + 1) as i64; // ceil(size/seats)
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works5() {
        let roads = vec![
            vec![1, 0],
            vec![0, 2],
            vec![2, 3],
            vec![0, 4],
            vec![5, 2],
            vec![6, 4],
            vec![3, 7],
            vec![7, 8],
            vec![9, 0],
        ];
        let result = minimum_fuel_cost(roads, 10);
        assert_eq!(result, 9);
    }

    #[test]
    fn it_works4() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let result = minimum_fuel_cost(roads, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works2() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let result = minimum_fuel_cost(roads, 2);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works() {
        let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let result = minimum_fuel_cost(roads, 5);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_works3() {
        let roads = vec![vec![0, 1]];
        let result = minimum_fuel_cost(roads, 5);
        assert_eq!(result, 1);
    }
}
