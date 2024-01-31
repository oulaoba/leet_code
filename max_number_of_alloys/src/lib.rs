pub fn max_number_of_alloys(
    n: i32,
    k: i32,
    budget: i32,
    composition: Vec<Vec<i32>>,
    stock: Vec<i32>,
    cost: Vec<i32>,
) -> i32 {
    let stocks = stock.iter().sum::<i32>();
    let mut ans = 0;
    for compos in composition {
        // 这台机器生产消耗的金属的消耗内容
        // 这台机器生产一次的总消耗
        let mut temp_stock = stock.clone();
        let mut money = budget;

        let mut this_max_cost = 0;
        for (i, c) in compos.iter().enumerate() {
            this_max_cost += c * cost[i]
        }
        let mut temp_stocks = stocks;

        let mut temp_ans = 0;
        while money > 0 || temp_stocks > 0 {
            let mut this_cost = this_max_cost;
            for (i, &c) in compos.iter().enumerate() {
                if temp_stock[i] >= c {
                    temp_stock[i] -= c;
                    temp_stocks -= c;
                    this_cost -= c * cost[i];
                } else if temp_stock[i] > 0 {
                    this_cost -= temp_stock[i] * cost[i];
                    temp_stocks -= temp_stock[i];
                    temp_stock[i] = 0;
                }
            }
            if money >= this_cost {
                money -= this_cost;
                temp_ans += 1;
            } else {
                break;
            }
            if this_cost == this_max_cost {
                break;
            }
        }
        temp_ans += money / this_max_cost;

        ans = ans.max(temp_ans);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
        let stock = vec![0, 0, 0];
        let cost = vec![1, 2, 3];
        let result = max_number_of_alloys(3, 2, 15, composition, stock, cost);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
        let stock = vec![0, 0, 100];
        let cost = vec![1, 2, 3];
        let result = max_number_of_alloys(3, 2, 15, composition, stock, cost);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works3() {
        let composition = vec![vec![2, 1], vec![1, 2], vec![1, 1]];
        let stock = vec![1, 1];
        let cost = vec![5, 5];
        let result = max_number_of_alloys(2, 3, 15, composition, stock, cost);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works4() {
        let composition = vec![
            vec![8, 3, 4, 2],
            vec![3, 9, 5, 5],
            vec![1, 7, 9, 8],
            vec![7, 6, 5, 1],
            vec![4, 6, 9, 4],
            vec![6, 8, 7, 1],
            vec![5, 10, 3, 4],
            vec![10, 1, 2, 4],
            vec![10, 3, 7, 2],
        ];
        let stock = vec![9, 1, 10, 0];
        let cost = vec![3, 4, 9, 9];
        let result = max_number_of_alloys(4, 9, 55, composition, stock, cost);
        assert_eq!(result, 1);
    }

    #[test]
    fn it_works5() {
        let composition = vec![vec![1]];
        let stock = vec![77472690];
        let cost = vec![1];
        let result = max_number_of_alloys(4, 9, 0, composition, stock, cost);
        assert_eq!(result, 77472690);
    }
}

pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
    let bigger = jug1_capacity.max(jug2_capacity);
    let smaller = jug1_capacity.min(jug2_capacity);
    let mut max = bigger + smaller;
    if max < target_capacity {
        return false;
    }
    while max > 0 {
        if target_capacity == max {
            return true;
        }
        if max >= smaller {
            max -= smaller;
        } else {
            max += bigger;
        }
    }
    false
}
