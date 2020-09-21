/// 函数中定义宏
pub fn macro_show() {
    macro_rules! calculate {
    ($e:expr) => {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
    };
  }
    calculate!(1 + 2 + 7);
    calculate!((1 + 2) * (8 / 4));
}

/// 定义宏并导出
#[macro_export]
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}
