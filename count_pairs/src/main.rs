fn main() {
    println!("count_pairs");
}

pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = 0;

    let n = nums.len();

    for i in 0..n {
        let iv = nums.get(i).unwrap();
        for j in (i..n).rev() {
            let jv = nums.get(j).unwrap();
            if iv + jv < target {
                ans += 1;
            }
        }
    }

    ans
}
