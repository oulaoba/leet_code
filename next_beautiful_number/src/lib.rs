pub fn next_beautiful_number(n: i32) -> i32 {
    fn is_balance(n: i32) -> bool {
        let mut step_balance = vec![0; 7];
        let mut n = n;
        while n > 0 {
            let mod_n = n % 10;
            if mod_n > 6 || mod_n == 0 {
                return false;
            }
            step_balance[mod_n as usize] += 1;
            n = n / 10
        }
        for i in 1..7 {
            if step_balance[i] != 0 && i != step_balance[i] {
                return false;
            }
        }
        true
    }
    let mut count = 1;
    loop {
        if is_balance(n + count) {
            return n + count;
        }
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = next_beautiful_number(2);
        assert_eq!(result, 22);
    }
}
