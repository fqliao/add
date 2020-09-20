#[derive(Debug)]
struct moves {
    a: i32,
    b: i32,
}

/// move是浅拷贝
pub fn test_move() {
    let mut moves1 = moves { a: 1, b: 2 };
    let p1 = &moves1 as *const moves;
    let mut moves2 = moves1;
    moves2.a = 9999;
    let p2 = &moves2 as *const moves;
    unsafe {
        println!("moves1 addr={:p}", p1);
    }
    unsafe {
        println!("moves2 addr={:p}", p2);
    }
    unsafe {
        println!("{}", (*p1).a);
    }
}
