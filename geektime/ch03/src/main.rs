#![allow(unused)]

use ch03::generic_demo;
use ch03::mutable_collection::cell::Foo;
use ch03::mutable_collection::refcell;
use ch03::struct_demo::name_struct::*;
use ch03::struct_demo::tuple_struct::*;
use ch03::struct_demo::unit_struct::*;
use ch03::trait_demo::*;
use std::cell::Cell;

fn main() {
    // test_struct();
    // test_mutable_collection();
    // test_generic_demo();
    test_trait_demo();
}

fn test_trait_demo() {
    // 调用A的函数
    A::hello();
    // 调用A实现的trait的函数，无歧义调用
    <A as DemoTrait>::hello();

    println!("{}", A::add(1, 2));
    println!("{}", B::add(1.1, 2.2));
    // trait多态调用
    println!("{}", <A as DemoTrait>::add(2, 3));
    println!("{}", <B as DemoTrait>::add(2.2, 3.3));
}

fn test_generic_demo() {
    let foo1 = generic_demo::foo(1);
    let foo2 = generic_demo::foo::<&'static str>("hello");
    println!("{}", foo1);
    println!("{}", foo2);
}

fn test_mutable_collection() {
    let foo = Foo {
        x: 1,
        y: Cell::new(2),
    };
    println!("{}", foo.x);
    let y = foo.y.get();
    println!("{}", y);
    foo.y.set(5);
    println!("{}", foo.y.get());

    let s = "hello".to_string();
    let str_cell = Cell::new(s);
    // into_inner将String从容器中提取出来，容器不能再使用了
    let s2 = str_cell.into_inner();
    println!("{:?}", s2);

    refcell::test_refcell();
}

fn test_struct() {
    // 打印Point结构体字段占用内存大小
    println!("{:?}", std::mem::size_of::<Point>());
    let point = Point::new(3.0, 4.0);
    println!("{:?}", point);
    println!("euclidean_distance:{}", point.euclidean_distance());

    let pair = Pair(1.1, 1.2);
    println!("{:?}", pair.0);

    let result = Score(90).pass();
    println!("{}", result);

    let unit1 = Unit;
    let unit2 = Unit;
    // 打印内存地址
    let unit1_addr = &unit1 as *const Unit as usize;
    let unit2_addr = &unit2 as *const Unit as usize;
    println!("unit1_addr:0x{:x}", unit1_addr);
    println!("unit2_addr:0x{:x}", unit2_addr);
}
