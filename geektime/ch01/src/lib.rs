//! # 第一章：Rust语言基础
//!
//!  ## 1主要是3种注释
//! 模块注释://！
//! 方法/结构体上的注释:///
//! 方法内的普通注释：//
//!
//! ## 2 定义宏
//! 宏接受参数的形式，常见如下几种：
//! - expr is used for expressions
//! - ident is used for variable/function names
//! - tt (token tree)
//! - ty (type)
//! - 其他见：https://doc.rust-lang.org/rust-by-example/macros/designators.html
//!
//! ## 3 定义结构体和trait
//! - 结构体内可以定义方法
//! - 结构体可以实现trait方法

pub mod lexical;
pub mod trait_demo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
