//! ## 5 rust编译器计算
//! ### 5.1 常量函数
//! CTFE(compile time function evaluation)最早由C++和Lisp语言支持
//! rust编译期计算支持：常量函数和常量泛型
//! Build脚本，例如生成c头文件的cbindgen库
//! 谨记：
//! 1. 常量上下文是编译器唯一运行进行编译期求值的地方
//! 2. 在非常量上下文的地方，常量表达式不一定会在编译期求值
//! MIR 解释器执行中间语言实现编译器计算
//!
//! ### 5.2 while true与loop 之争
//! while true其实属于一种特殊的情况，更为普遍的是 while (constexpr == true) 的情况，
//! 如果后者的条件表达式越复杂越难判断到底是不是 true。
//! 为了保持语言的一致性，就不能给 while true 开小灶。
//! 使用 #[allow(while_true)] 属性在某些情况下允许使用 while true。
//!
//! ### 5.3 常量泛型
//! Rust 中的静态数组一直以来都属于「二等公民」，不方便使用，常量泛型可以
//!

pub mod ctfe;
