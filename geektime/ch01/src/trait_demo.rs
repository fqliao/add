/// 结构体S
pub struct S;

/// S的内置方法
impl S {
    pub fn f() {
        println!("S f");
    }
}

/// trait T1
/// 定义方法f
pub trait T1 {
    /// f函数
    /// # Examples
    /// ```
    /// <S as T1>::f()
    /// ```
    fn f();
}

/// trait T2
/// 定义方法f
pub trait T2 {
    /// f函数
    /// # Examples
    /// ```
    /// <S as T2>::f()
    /// ```
    fn f();
}

/// S实现trait T1中的接口
impl T1 for S {
    fn f() {
        println!("T1 f");
    }
}
/// S实现trait T2中的接口
impl T2 for S {
    fn f() {
        println!("T2 f");
    }
}
