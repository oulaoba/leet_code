pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let row = heights.len();
    let col = heights[0].len();
    let mut map = vec![vec![0; col]; row];
    let mut set: std::collections::BTreeMap<i32, Vec<(usize, usize)>> =
        std::collections::BTreeMap::new();
    for i in 0..row {
        for j in 0..col {
            let mut cost = i32::MAX;
            //上
            if i > 0 {
                cost = cost.min((heights[i - 1][j] - heights[i][j]).abs());
            }
            //下
            if i < row - 1 {
                cost = cost.min((heights[i + 1][j] - heights[i][j]).abs());
            }
            //左
            if j > 0 {
                cost = cost.min((heights[i][j - 1] - heights[i][j]).abs());
            }
            //右
            if j < col - 1 {
                cost = cost.min((heights[i][j + 1] - heights[i][j]).abs());
            }
            map[i][j] = cost;
            set.entry(cost)
                .and_modify(|f| f.push((i, j)))
                .or_insert(vec![(i, j)]);
        }
    }
    let mut connection_map = std::collections::HashSet::new();
    while let Some(val) = set.pop_first() {
        connection_map.extend(val.1);
        if connection_map.len() < row + col - 1
            || !connection_map.contains(&(0, 0))
            || !connection_map.contains(&(row - 1, col - 1))
        {
            // 当前元素如果不大于最短路径，则肯定无法到达
            continue;
        } else {
            // 初始坐标
            let connection_test = connection_map.clone();
            if is_connected(connection_test, row, col) {
                ans = val.0;
            }
        }
    }

    fn is_connected(
        mut connection_test: std::collections::HashSet<(usize, usize)>,
        row: usize,
        col: usize,
    ) -> bool {
        let (mut x, mut y) = (0, 0);
        connection_test.remove(&(x, y));
        while x != row - 1 && y != col - 1 {
            match (
                connection_test.contains(&(x + 1, y)),
                connection_test.contains(&(x, y + 1)),
            ) {
                (true, true) => {
                    connection_test.remove(&(x + 1, y));
                }
                (true, false) => {
                    connection_test.remove(&(x + 1, y));
                }
                (false, true) => {
                    connection_test.remove(&(x, y + 1));
                }
                (false, false) => {
                    return false;
                }
            }
        }
        x == row - 1 && y == col - 1
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        let result = minimum_effort_path(heights);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        let result = minimum_effort_path(heights);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works3() {
        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        let result = minimum_effort_path(heights);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works4() {
        let heights = vec![vec![1]];
        let result = minimum_effort_path(heights);
        assert_eq!(result, 0);
    }
}
