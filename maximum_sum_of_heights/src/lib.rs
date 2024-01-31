pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let mut ans = 0;
    let n = max_heights.len();
    let mut prefix = vec![0; n];
    let mut suffix = vec![0; n];

    let mut stack1 = vec![];
    let mut stack2 = vec![];

    for i in 0..n {
        while stack1.len() > 0 && max_heights[i] < max_heights[stack1[stack1.len() - 1]] {
            stack1.pop();
        }
        if stack1.is_empty() {
            prefix[i] = (i + 1) * max_heights[i] as usize;
        } else {
            prefix[i] =
                prefix[stack1.len() - 1] + (i - stack1[stack1.len() - 1]) * max_heights[i] as usize;
        }
        stack1.push(i);
    }
    for i in (0..n).rev() {
        while stack2.len() > 0 && max_heights[i] < max_heights[stack2[stack2.len() - 1]] {
            stack2.pop();
        }
        if stack2.len() == 0 {
            suffix[i] = (n - i) * max_heights[i] as usize;
        } else {
            suffix[i] = suffix[stack2[stack2.len() - 1]]
                + (stack2[stack2.len() - 1] - i) * max_heights[i] as usize;
        }
        stack2.push(i);
        ans = ans.max(prefix[i] + suffix[i] - max_heights[i] as usize);
    }

    ans as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let max_heights = vec![5, 3, 4, 1, 1];
        let result = maximum_sum_of_heights(max_heights);
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works2() {
        let max_heights = vec![6, 5, 3, 9, 2, 7];
        let result = maximum_sum_of_heights(max_heights);
        assert_eq!(result, 22);
    }

    #[test]
    fn it_works3() {
        let max_heights = vec![3, 2, 5, 5, 2, 3];
        let result = maximum_sum_of_heights(max_heights);
        assert_eq!(result, 18);
    }
}