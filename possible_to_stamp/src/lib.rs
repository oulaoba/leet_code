pub fn possible_to_stamp1(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();

    let mut w = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                w += 1;
            } else {
                if w != 0 && w < stamp_width {
                    return false;
                }
                w = 0;
            }
        }
        if w != 0 && w < stamp_width {
            return false;
        }
        w = 0;
    }

    let mut h = 0;
    for j in 0..n {
        for i in 0..m {
            if grid[i][j] == 0 {
                h += 1;
            } else {
                if h != 0 && h < stamp_height {
                    return false;
                }
                h = 0;
            }
        }
        if h != 0 && h < stamp_height {
            return false;
        }
        h = 0;
    }

    true
}

pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();

    let mut s = vec![vec![0; n + 1]; m + 1];
    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + v;
        }
    }

    let mut d = vec![vec![0; n + 2]; m + 2];
    for i2 in stamp_height as usize..=m {
        for j2 in stamp_width as usize..=n {
            let i1 = i2 - stamp_height as usize + 1;
            let j1 = j2 - stamp_width as usize + 1;
            if s[i2][j2] - s[i2][j1 - 1] - s[i1 - 1][j2] + s[i1 - 1][j1 - 1] == 0 {
                d[i1][j1] += 1;
                d[i1][j2 + 1] -= 1;
                d[i2 + 1][j1] -= 1;
                d[i2 + 1][j2 + 1] += 1;
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            d[i + 1][j + 1] += d[i + 1][j] + d[i][j + 1] - d[i][j];
            if v == 0 && d[i + 1][j + 1] == 0 {
                return false;
            }
        }
    }

    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let result = possible_to_stamp(grid, 4, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let result = possible_to_stamp(grid, 2, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn it_works2() {
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 1],
        ];
        let result = possible_to_stamp(grid, 2, 2);
        assert_eq!(result, false);
    }
}
