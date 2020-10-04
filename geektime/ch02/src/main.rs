#![allow(unused)]

use ch02::expression01::skeleton;
use ch02::expression02::ctfe;
use ch02::expression03;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

fn main() {
    // test_expression01();
    // test_expression02();
    test_expression03();
}

fn test_expression03() {
    // expression03::if_let();
    // expression03::stack_heap();
    expression03::mut_ref();
}

fn test_expression02() {
    let start = SystemTime::now();
    // 使用运行时计算
    // let result = ctfe::gcd(1234567890124324,987654321034365467);
    let result = ctfe::gcd_euclid(1234567890124324, 987654321034365467);
    // 使用编译期计算
    // let result = ctfe::GCD;
    let time = SystemTime::now().duration_since(start).unwrap().as_nanos();
    println!("{:?}", result);
    println!("time: {:?}", time);

    let start = SystemTime::now();
    // 使用运行时计算
    let n = ctfe::fib(100);
    // 使用编译期计算
    // let n = ctfe::N;
    let time = SystemTime::now().duration_since(start).unwrap().as_nanos();
    println!("{}", n);
    println!("time: {:?}", time);

    let answer = ctfe::get_a();
    println!("{:?}", answer.get_first_value());

    println!("i:{}", ctfe::infinite_loop());
}

fn test_expression01() {
    skeleton::semicolon();

    let mut a = 41;
    skeleton::add_one(&mut a);
    println!("a:{}", a);

    let a = 41;
    let b = skeleton::plus_one(&a);
    println!("b:{}", b);

    let mut map = HashMap::new();
    map.insert("answer", 42);
    let value = map["answer"];
    println!("answer is {:?}", value);
    let option_value = map.get("answer1");
    // println!("answer is {:?}", option_value.unwrap());
    match option_value {
        Some(value) => println!("answer is {:?}", value),
        None => println!("there is no answer"),
    }
    option_value.map(|value| println!("answer is {:?}", value));

    // skeleton::if_expression();
    // skeleton::match_expression();
    skeleton::if_let_expression();
}
