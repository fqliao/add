pub fn fibonacci(n: u64) -> u64 {
    //    /// slow way
    //    match n {
    //        0 => 1,
    //        1 => 1,
    //        n => fibonacci(n - 1) + fibonacci(n - 2),
    //    }

    /// fast way
    let mut a = 0u64;
    let mut b = 1u64;
    let mut c = 0u64;

    if n == 0 {
        return 0;
    }

    for _ in 0..(n + 1) {
        c = a + b;
        a = b;
        b = c;
        //        println!("{}", b);
    }
    return b;
}
