use std::cell::Cell;

/// trait 是行为的抽象
/// Rust 引入 trait 来统一行为接口，管控类型的行为，实现多态功能
/// trait 是行为的抽象
/// trait 是一种特设多态，即Ad-hoc 多态，一个接口，多个实现
/// 孤儿规则： trait 或 类型，必须有一个在本地定义
pub struct A;
impl A {
    pub fn hello() {
        println!("in A");
    }
}

pub struct B;
impl B {
    pub fn hello() {
        println!("in B");
    }
}

pub trait DemoTrait<Rhs = Self> {
    // 定义类型，实现该trait的结构体也要实现,这里需要trait定义<Rhs = Self>进行配合
    type AddFnType;
    fn hello();
    // 注意，这里要用Self::Output 不能用Output
    fn add(a: Self::AddFnType, b: Self::AddFnType) -> Self::AddFnType;
}

impl DemoTrait for A {
    // 实现类型为i32
    type AddFnType = i32;

    fn hello() {
        println!("A from Hello trait");
    }
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

impl DemoTrait for B {
    // 实现类型为f32
    type AddFnType = f32;

    fn hello() {
        println!("B from Hello trait");
    }
    fn add(a: f32, b: f32) -> f32 {
        a + b
    }
}
