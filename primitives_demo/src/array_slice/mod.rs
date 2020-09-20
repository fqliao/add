use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

/// 1 数组存储相同类型
/// 2 可以显示声明类型[type;size]
/// 3 访问元素方式[]
/// 4 数组长度len()
pub fn test_array_slice() {
    let xs = [1, 2, 3, 4, 5];
    println!("xs:{:?}", xs);
    println!("xs[0]:{:?}", xs[0]);

    // 数组长度超过32，则不能直接打印
    let ys: [i32; 500] = [0; 500];
    println!("ys:{}", ys[499]);
    println!("ys.len:{}", ys.len());
    println!("array occupy {} bytes", mem::size_of_val(&ys));

    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);
}
