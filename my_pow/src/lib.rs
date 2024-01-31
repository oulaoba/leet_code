pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut ans: f64 = 1.0;
    fn quick_mul(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        } else {
            let y = quick_mul(x, n / 2);
            if n % 2 == 0 {
                y * y
            } else {
                y * y * x
            }
        }
    }
    ans *= quick_mul(x, n);
    if n < 0 {
        ans = 1.0 / ans;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = my_pow(2.0, 2);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn it_works2() {
        let result = my_pow(2.0, 10);
        assert_eq!(result, 1024.0);
    }
    #[test]
    fn it_works3() {
        let result = my_pow(2.1, 3);
        assert_eq!(result, 9.26100);
    }
    #[test]
    fn it_works4() {
        let result = my_pow(2.0, -2);
        assert_eq!(result, 0.25000);
    }
}
