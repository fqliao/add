pub fn test_literals() {
    println!("1 + 2 = {}", 1 + 2_u32);
    // 变为u32则错误
    println!("1 - 2 = {}", 1_i32 - 2);

    // 逻辑运算符 短路逻辑
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    // 位运算
    println!("0011 and 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);

    println!("one million is written as {}", 1_000_000_u32);
}
