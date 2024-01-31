fn main() {
    println!("num_prime_arrangements");
}

pub fn num_prime_arrangements(n: i32) -> i32 {
    const MOD: i128 = 1000000007;
    pub fn factorial(n: i32) -> i128 {
        let mut ans: i128 = 1;
        for i in 1..=n {
            ans *= i as i128;
            ans %= MOD
        }
        ans
    }

    pub fn prime_count(n: i32) -> i32 {
        if n < 3 {
            1
        } else if n < 5 {
            2
        } else if n < 7 {
            3
        } else if n < 11 {
            4
        } else if n < 13 {
            5
        } else if n < 17 {
            6
        } else if n < 19 {
            7
        } else if n < 23 {
            8
        } else if n < 29 {
            9
        } else if n < 31 {
            10
        } else if n < 37 {
            11
        } else if n < 41 {
            12
        } else if n < 43 {
            13
        } else if n < 47 {
            14
        } else if n < 53 {
            15
        } else if n < 59 {
            16
        } else if n < 61 {
            17
        } else if n < 67 {
            18
        } else if n < 71 {
            19
        } else if n < 73 {
            20
        } else if n < 79 {
            21
        } else if n < 83 {
            22
        } else if n < 89 {
            23
        } else if n < 97 {
            24
        } else {
            25
        }
    }

    let prime_count = prime_count(n);
    let ans = factorial(prime_count) * factorial(n - prime_count) % MOD;
    ans as i32
}
