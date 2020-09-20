use std::mem;

pub fn test_literals() {
    let x = 1_u8;
    let y = 2_u16;
    let z = 3_f32;
    println!("x->y-z: {} -> {} -> {}", x, y, z);

    let i = 1;
    let f = 1.0;
    let c = 48 as char;
    let g = 1000000_u128;
    let s = 1000000_usize;
    let is = 1000000_isize;
    println!("size of x in bytes: {}", mem::size_of_val(&x));
    println!("size of y in bytes: {}", mem::size_of_val(&y));
    println!("size of z in bytes: {}", mem::size_of_val(&z));
    println!("size of i in bytes: {}", mem::size_of_val(&i));
    println!("size of f in bytes: {}", mem::size_of_val(&f));
    println!("size of c in bytes: {}", mem::size_of_val(&c));
    println!("size of usize in bytes: {}", mem::size_of_val(&s));
    println!("size of isize in bytes: {}", mem::size_of_val(&is));
}
