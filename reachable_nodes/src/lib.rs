pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let dic = restricted
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    let mut g = vec![vec![]; n as usize];
    for e in &edges {
        let x = e[0];
        let y = e[1];
        if !dic.contains(&x) && !dic.contains(&y) {
            g[x as usize].push(y as usize);
            g[y as usize].push(x as usize);
        }
    }

    fn dfs(x: usize, fa: usize, g: &Vec<Vec<usize>>) -> i32 {
        let mut cnt = 1;
        for &y in &g[x] {
            if y != fa {
                cnt += dfs(y, x, g);
            }
        }
        cnt
    }
    dfs(0, 0, &g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 1],
            vec![4, 0],
            vec![0, 5],
            vec![5, 6],
        ];
        let restricted = vec![4, 5];
        let result = reachable_nodes(7, edges, restricted);
        assert_eq!(result, 4);
    }
}

struct MyStack {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut temp = vec![];
        let ans = self.data.pop().unwrap();
        for i in 0..self.data.len() - 1 {
            temp.push(self.data[i]);
        }
        self.data = temp;
        ans
    }

    fn top(&self) -> i32 {
        self.data[self.data.len() - 1]
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

struct MyQueue {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x)
    }

    fn pop(&mut self) -> i32 {
        let mut temp = vec![];
        for i in 1..self.data.len() {
            temp.push(self.data[i])
        }
        let ans = self.data[0].clone();
        self.data = temp;
        ans
    }

    fn peek(&self) -> i32 {
        let mut ans = 0;
        for i in (0..self.data.len()).rev() {
            ans = self.data[i];
        }
        ans
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}
