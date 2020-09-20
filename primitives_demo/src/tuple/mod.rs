use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a, b) = pair;
    (b, a)
}

// 这是结构体元组，给元组定了一个具体类型
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n ({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

/// 1 元组方便组合多个相同或不同的值
/// 2 返回多个值，使用元组比较合适
/// 3 元素个数不要超过12个，超过说明设计不合理，打印都不支持。可以组合元组完成。
/// 4 一个元素的元组需要使用逗号，不然是一个单独元素类别
/// 5 访问元素.index
/// 6 可以解析元素到对应变量
pub fn test_tuple() {
    let pair = (1, true);
    let result = reverse(pair);
    println!("result:{:?}", result);

    let long_tuple = (
        1_u8, 2_u16, 3_u32, 4_u64, -1_i8, -2_i16, -3_i32, -4_i64, 0.1_f32, 0.2_f64, 'a', true,
    );
    println!("long tuple first value:{:?}", long_tuple.0);
    println!("long tuple second value:{:?}", long_tuple.1);

    let tuple_of_tuple = ((1_u8, 2_u16, 2_u32), (4_u64, -1i8), -2_i16);
    println!("tuple of tuple:{:?}", tuple_of_tuple);

    // 元素超过12个，不能打印。可以较少个数，组合tuple即可
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, (12, 13));
    println!("too long tuple:{:?}", too_long_tuple);

    println!("one element tuple: {:?}", (5_u32,));
    println!("one integer: {:?}", (5_u32));

    let binding_variable = (1, "hello", 4.5, true);
    let (a, b, c, d) = binding_variable;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{:?}", matrix.0);

    let m = transpose(matrix);
    println!("{}", m);
}
