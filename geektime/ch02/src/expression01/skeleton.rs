#![allow(unused)]
use std::collections::HashMap;

// 一个分号也是一个表达式
pub fn semicolon() {}

pub fn add_one(i: &mut u32) {
    *i += 1;
}

pub fn plus_one(i: &u32) -> u32 {
    let i = i + 1;
    i
}

pub fn if_expression() {
    for i in 0..=102 {
        if i % 15 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("i:{:?}", i)
        }
    }
}

pub fn match_expression() {
    for i in 0..=102 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("i:{:?}", i),
        }
    }
}

// if let中let后面是模式匹配，如果只是if，则不是模式匹配，只能判断关系式的值
pub fn if_let_expression() {
    let dish = ("Ham", "Eggs");

    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    } else {
        println!("No bacon will be served");
    }

    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    } else {
        println!("No Ham will be served");
    }
}
