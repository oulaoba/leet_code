fn main() {
    println!("min_path_cost");
}
/*
给你一个下标从 0 开始的整数矩阵 grid ，矩阵大小为 m x n ，由从 0 到 m * n - 1 的不同整数组成。你可以在此矩阵中，从一个单元格移动到 下一行 的任何其他单元格。如果你位于单元格 (x, y) ，且满足 x < m - 1 ，你可以移动到 (x + 1, 0), (x + 1, 1), ..., (x + 1, n - 1) 中的任何一个单元格。注意： 在最后一行中的单元格不能触发移动。

每次可能的移动都需要付出对应的代价，代价用一个下标从 0 开始的二维数组 moveCost 表示，该数组大小为 (m * n) x n ，其中 moveCost[i][j] 是从值为 i 的单元格移动到下一行第 j 列单元格的代价。从 grid 最后一行的单元格移动的代价可以忽略。

grid 一条路径的代价是：所有路径经过的单元格的 值之和 加上 所有移动的 代价之和 。从 第一行 任意单元格出发，返回到达 最后一行 任意单元格的最小路径代价。
*/

pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut memo = vec![vec![0; n]; m];
    fn dfs(
        i: usize,
        j: usize,
        memo: &mut Vec<Vec<i32>>,
        grid: &Vec<Vec<i32>>,
        move_cost: &Vec<Vec<i32>>,
    ) -> i32 {
        if i == grid.len() - 1 {
            // 递归边界
            return grid[i][j];
        }
        if memo[i][j] != 0 {
            return memo[i][j];
        }
        let mut res = i32::MAX;
        for (k, &c) in move_cost[grid[i][j] as usize].iter().enumerate() {
            // 移动到下一行的第 k 列
            res = res.min(dfs(i + 1, k, memo, grid, move_cost) + c);
        }
        res += grid[i][j];
        memo[i][j] = res;
        res
    }
    let mut ans = i32::MAX;
    for j in 0..grid[0].len() {
        // 枚举起点
        ans = ans.min(dfs(0, j, &mut memo, &grid, &move_cost));
    }
    ans
}
