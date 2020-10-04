//! 使用无限循环的时候，建议你使用 loop，而非 while true

pub fn if_let() {
    let dish = ["Bacon", "Eggs"];
    if let ["Bacon", b] = dish {
        println!("Bacon is served with {}", b);
    } else {
        println!("No bacon will be served");
    }
}

pub fn stack_heap() {
    // 整型实现了Copy trait是copy
    let stack_a = 12;
    let stack_b = stack_a;
    println!("stack_a:{}", stack_a);
    println!("stack_b:{}", stack_b);

    // String类型未实现Copy，是move语义，因此heap_a的所有权被移动了，heap_a失效
    let heap_a = "hello".to_string();
    let heap_b = heap_a;
    // println!("heap_a:{}", heap_a);
    println!("heap_b:{}", heap_b);
}

/// & 共享引用 不可变引用
/// &mut 独占引用  可变引用
pub fn mut_ref() {
    let mut a = 12;
    let b = &mut a;
    *b = 15;
    // a 和 b不能同时使用，只能用一个，因为b是独占引用，有a无b，有b无a
    println!("a:{}", a);
    // println!("b:{}", b);
}
