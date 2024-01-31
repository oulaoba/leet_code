pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut raw = vec![0; m];
    let mut col = vec![0; n];
    let mut map = std::collections::HashMap::new();

    for i in 0..m {
        for j in 0..n {
            map.insert(mat[i][j], (i, j));
        }
    }

    for i in 0..arr.len() {
        let current = arr[i];
        if let Some(v) = map.get(&current) {
            raw[v.0] += 1;
            col[v.1] += 1;
            match (raw[v.0] == n, col[v.1] == m) {
                (false, false) => (),
                _ => return i as i32,
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 3, 2, 4];
        let mat = vec![vec![1, 4], vec![2, 3]];
        let result = first_complete_index(arr, mat);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let arr = vec![1, 4, 5, 2, 6, 3];
        let mat = vec![vec![4, 3, 5], vec![1, 2, 6]];
        let result = first_complete_index(arr, mat);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works3() {
        let arr = vec![1, 4, 5, 2, 6, 3];
        let mat = vec![vec![4, 3], vec![1, 2], vec![6, 5]];
        let result = first_complete_index(arr, mat);
        assert_eq!(result, 3);
    }
}
