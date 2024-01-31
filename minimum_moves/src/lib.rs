pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut ans3 = 0;
    let mut ans4 = 0;
    let mut arr1 = vec![
        grid[0][0], grid[0][1], grid[0][2], grid[1][2], grid[2][2], grid[2][1], grid[2][0],
        grid[1][0],
    ];
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    // 顺时针 前后
    ans1 += search(&mut arr1, 1, true, true);
    // 顺时针 后前
    ans2 += search(&mut arr2, 1, false, true);
    // 逆时针 前后
    ans3 += search(&mut arr3, 1, true, false);
    // 逆时针 后前
    ans4 += search(&mut arr4, 1, false, false);

    ans1 += search(&mut arr1, 2, true, true);
    ans2 += search(&mut arr2, 2, false, true);
    ans3 += search(&mut arr3, 2, true, false);
    ans4 += search(&mut arr4, 2, false, false);

    ans1 += finish(&arr1);
    ans2 += finish(&arr2);
    ans3 += finish(&arr3);
    ans4 += finish(&arr4);

    ans1.min(ans2).min(ans3).min(ans4)
}

fn finish(arr: &Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..8 {
        let step = match (i % 2) == 0 {
            true => 2,
            false => 1,
        };
        if arr[i] == 0 {
            ans += step;
        }
        if arr[i] > 1 {
            ans += (arr[i] - 1) * step
        }
    }
    ans
}

fn search(arr: &mut Vec<i32>, step: usize, first: bool, front: bool) -> i32 {
    let mut ans = 0;
    let count = arr.len();

    for i in count..count * 2 {
        match first {
            true => match front {
                true => {
                    if arr[i % count] > 1 && arr[(i + step) % count] == 0 {
                        arr[i % count] = arr[i % count] - 1;
                        arr[(i + step) % count] = 1;
                        ans += 1;
                    }
                    if arr[i % count] > 1 && arr[(i - 1) % count] == 0 {
                        arr[i % count] = arr[i % count] - 1;
                        arr[(i - 1) % count] = 1;
                        ans += 1;
                    }
                }
                false => {
                    let current = (count - i % count) % count;
                    let current_step = (count - (i + step) % count) % count;
                    if arr[current] > 1 && arr[(i + step) % count] == 0 {
                        arr[current] = arr[current] - 1;
                        arr[current_step] = 1;
                        ans += 1;
                    }
                    if arr[current] > 1 && arr[(i - 1) % count] == 0 {
                        arr[current] = arr[current] - 1;
                        arr[current_step] = 1;
                        ans += 1;
                    }
                }
            },
            false => match front {
                true => {
                    if arr[i % count] > 1 && arr[(i - 1) % count] == 0 {
                        arr[i % count] = arr[i % count] - 1;
                        arr[(i - 1) % count] = 1;
                        ans += 1;
                    }
                    if arr[i % count] > 1 && arr[(i + step) % count] == 0 {
                        arr[i % count] = arr[i % count] - 1;
                        arr[(i + step) % count] = 1;
                        ans += 1;
                    }
                }
                false => {
                    let current = (count - i % count) % count;
                    let current_step = (count - (i + step) % count) % count;
                    if arr[current] > 1 && arr[(i - 1) % count] == 0 {
                        arr[current] = arr[current] - 1;
                        arr[current_step] = 1;
                        ans += 1;
                    }
                    if arr[current] > 1 && arr[(i + step) % count] == 0 {
                        arr[current] = arr[current] - 1;
                        arr[current_step] = 1;
                        ans += 1;
                    }
                }
            },
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_moves_it_works6() {
        let grid = vec![vec![0, 0, 0], vec![1, 2, 0], vec![3, 0, 3]];
        let result = minimum_moves(grid);
        assert_eq!(result, 7);
    }

    #[test]
    fn minimum_moves_it_works5() {
        let grid = vec![vec![1, 0, 0], vec![4, 1, 1], vec![0, 2, 0]];
        let result = minimum_moves(grid);
        assert_eq!(result, 7);
    }

    #[test]
    fn minimum_moves_it_works() {
        let grid = vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]];
        let result = minimum_moves(grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn minimum_moves_it_works2() {
        let grid = vec![vec![2, 0, 2], vec![0, 1, 0], vec![1, 2, 1]];
        let result = minimum_moves(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn minimum_moves_it_works3() {
        let grid = vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]];
        let result = minimum_moves(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn minimum_moves_it_works4() {
        let grid = vec![vec![3, 1, 0], vec![1, 0, 0], vec![1, 0, 3]];
        let result = minimum_moves(grid);
        assert_eq!(result, 6);
    }
}

pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..mountain.len() - 1 {
        if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
            ans.push(i as i32);
        }
    }
    ans
}

pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
    let mut set = std::collections::HashSet::new();
    for i in 0..coins.len() {
        set.insert(coins[i]);
    }
    let mut coins = coins.clone();
    0
}

#[test]
fn test1() {
    let coins = vec![1, 4, 10];
    let ans = minimum_added_coins(coins, 19);
    assert_eq!(2, ans);
}
