pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut balance = 0;
    let mut sub_balance = 0;

    for i in 0..gas.len() {
        let fuel = gas[i] - cost[i];
        balance += fuel;
        sub_balance += fuel;
        if sub_balance < 0 {
            ans = i + 1;
            sub_balance = 0;
        }
    }
    if 0 > balance {
        -1
    } else {
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works4() {
        let gas = vec![4];
        let cost = vec![4];
        let result = can_complete_circuit(gas, cost);
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works3() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![4, 4, 1, 5, 1];
        let result = can_complete_circuit(gas, cost);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let result = can_complete_circuit(gas, cost);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        let result = can_complete_circuit(gas, cost);
        assert_eq!(result, -1);
    }
}
