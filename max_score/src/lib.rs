
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut init: i32 = card_points[0..k].iter().sum();
        let mut ans = init;
        let n = card_points.len();
        for i in 0..k {
            init += card_points[n - 1 - i];
            init -= card_points[k - 1 - i];
            ans = ans.max(init);
        }
        ans
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works6() {
        let card_points = vec![96, 90, 41, 82, 39, 74, 64, 50, 30];
        let result = max_score(card_points, 8);
        assert_eq!(result, 536);
    }

    #[test]
    fn it_works5() {
        let card_points = vec![11, 49, 100, 20, 86, 29, 72];
        let result = max_score(card_points, 4);
        assert_eq!(result, 232);
    }

    #[test]
    fn it_works() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let result = max_score(card_points, 3);
        assert_eq!(result, 12);
    }
    #[test]
    fn it_works2() {
        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let result = max_score(card_points, 7);
        assert_eq!(result, 55);
    }

    #[test]
    fn it_works3() {
        let card_points = vec![1, 1000, 1];
        let result = max_score(card_points, 1);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_works4() {
        let card_points = vec![1, 79, 80, 1, 1, 1, 200, 1];
        let result = max_score(card_points, 3);
        assert_eq!(result, 202);
    }
}
