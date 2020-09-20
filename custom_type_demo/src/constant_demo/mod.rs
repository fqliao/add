use std::collections::HashMap;

const THRESHOLD: i32 = 10;
static LANGUAGE: &str = "Rust";

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
/// lazy_static!适合加载对上分配的对象
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
/// rust有两类常量
/// 1 const 值不可改变
/// 2 static 局部static声明周期，修改需要放在unsafe代码块，不建议使用mut修饰，使其可变。
/// 对于初始化大对象，可以使用static懒加载模式
/// 定义常需要显示表名类型
pub fn test_constant() {
    let n = 12;
    println!("{} is big? {}", n, is_big(n));
    println!("The language is {}", LANGUAGE);
    println!("first value is {}", HASHMAP.get(&0).unwrap());
}
