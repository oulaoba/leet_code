pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut map = vec![vec![]; n + 1];
    for ride in rides {
        let (from, to, tip) = (ride[0], ride[1], ride[2]);
        map[to as usize].push((from, tip + to - from));
    }
    let mut memo = vec![-1_i64; n + 1];
    fn dfs(index: usize, memo: &mut Vec<i64>, map: &Vec<Vec<(i32, i32)>>) -> i64 {
        if index == 0 {
            return 0;
        }
        if memo[index] != -1 {
            return memo[index];
        }
        let mut res = dfs(index - 1, memo, map);
        for index_money in &map[index] {
            res = res.max(dfs(index_money.0 as usize, memo, map) + index_money.1 as i64);
        }
        res
    }
    dfs(n, &mut memo, &map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let rides = vec![
            vec![1, 6, 1],
            vec![3, 10, 2],
            vec![10, 12, 3],
            vec![11, 12, 2],
            vec![12, 15, 2],
            vec![13, 18, 1],
        ];
        let result = max_taxi_earnings(20, rides);
        assert_eq!(result, 20);
    }

    #[test]
    fn it_works1() {
        let rides = vec![vec![2, 5, 4], vec![1, 5, 1]];
        let result = max_taxi_earnings(5, rides);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works() {
        let rides = vec![
            vec![62, 66, 73],
            vec![93, 96, 24],
            vec![36, 56, 38],
            vec![72, 92, 25],
            vec![32, 50, 30],
            vec![9, 56, 10],
            vec![93, 96, 26],
            vec![18, 24, 43],
            vec![91, 98, 97],
            vec![13, 70, 84],
            vec![31, 96, 86],
            vec![4, 19, 39],
            vec![25, 70, 75],
            vec![68, 79, 11],
            vec![66, 89, 30],
            vec![85, 98, 96],
            vec![66, 72, 20],
            vec![46, 100, 16],
            vec![5, 57, 62],
            vec![70, 71, 96],
            vec![20, 56, 91],
            vec![12, 19, 87],
            vec![15, 92, 33],
            vec![12, 46, 14],
            vec![16, 83, 38],
            vec![73, 74, 68],
            vec![99, 100, 69],
            vec![69, 97, 40],
            vec![21, 83, 15],
            vec![70, 83, 95],
            vec![91, 92, 48],
            vec![94, 98, 84],
            vec![2, 56, 84],
            vec![71, 85, 51],
            vec![65, 73, 9],
            vec![65, 72, 26],
            vec![32, 91, 99],
            vec![34, 95, 9],
            vec![57, 73, 47],
            vec![49, 52, 81],
            vec![29, 90, 66],
            vec![41, 79, 30],
            vec![80, 85, 26],
            vec![56, 74, 94],
            vec![83, 92, 56],
            vec![95, 96, 95],
            vec![46, 99, 10],
            vec![48, 61, 23],
            vec![80, 96, 33],
            vec![66, 85, 29],
            vec![21, 71, 69],
            vec![67, 73, 16],
            vec![37, 81, 17],
            vec![15, 21, 49],
            vec![65, 83, 30],
            vec![60, 81, 95],
            vec![52, 98, 9],
            vec![67, 75, 14],
            vec![98, 100, 3],
            vec![90, 100, 20],
            vec![6, 57, 64],
            vec![43, 53, 87],
            vec![70, 88, 44],
            vec![57, 61, 13],
            vec![36, 41, 25],
            vec![96, 100, 28],
            vec![34, 75, 61],
            vec![42, 61, 85],
            vec![12, 56, 19],
            vec![35, 83, 98],
            vec![33, 66, 37],
            vec![86, 92, 83],
            vec![41, 88, 11],
            vec![3, 71, 32],
            vec![34, 86, 54],
            vec![38, 82, 28],
            vec![65, 69, 12],
            vec![28, 38, 59],
            vec![67, 99, 46],
            vec![51, 97, 39],
            vec![86, 97, 65],
            vec![38, 42, 38],
            vec![49, 75, 62],
            vec![45, 46, 21],
            vec![33, 92, 93],
            vec![56, 66, 100],
            vec![41, 49, 76],
            vec![69, 77, 75],
            vec![65, 67, 18],
            vec![87, 97, 42],
            vec![89, 96, 100],
            vec![16, 55, 51],
            vec![25, 70, 1],
            vec![26, 98, 3],
            vec![48, 87, 34],
            vec![7, 95, 6],
            vec![94, 95, 73],
            vec![84, 91, 59],
            vec![57, 70, 71],
            vec![5, 90, 53],
        ];
        let result = max_taxi_earnings(100, rides);
        assert_eq!(result, 7);
    }
}
