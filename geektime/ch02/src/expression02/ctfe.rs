pub const fn gcd(a: u64, b: u64) -> u64 {
    // println!("a:{}, b:{}", a, b);
    match (a, b) {
        (x, 0) | (0, x) => x,
        (x, y) if x % 2 == 0 && y % 2 == 0 => 2 * gcd(x / 2, y / 2),
        (x, y) | (y, x) if x % 2 == 0 => gcd(x / 2, y),
        (x, y) if x < y => gcd((y - x) / 2, x),
        (x, y) => gcd((x - y) / 2, y),
    }
}

// rust没有三元运算符，也不需要，因为表达式都可以求值
pub const fn gcd_euclid(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub const GCD: u64 = gcd(1234567890124324, 987654321034365467);

pub const fn fib(n: u128) -> u128 {
    const fn helper(n: u128, a: u128, b: u128, i: u128) -> u128 {
        if i <= n {
            helper(n, b, a + b, i + 1)
        } else {
            b
        }
    }
    helper(n, 1, 1, 2)
}

pub const N: u128 = fib(100);

pub const fn hell0() -> &'static str {
    "hello"
}

#[derive(Debug)]
pub struct Answer(u32);
impl Answer {
    pub fn get_first_value(&self) -> u32 {
        self.0
    }
}

pub const A: Answer = Answer(32);

pub const fn get_a() -> Answer {
    A
}

pub fn infinite_loop() -> i32 {
    let mut i = 0;
    loop {
        i += 1;
        break;
    }
    i
}
