use error_handling::{combinators, option_unwrap, question_mark, struct_demo};

/// main也可以返回Result类型 例如Result<(), Box<dyn Error>>
/// Box<dyn Error> 为 “任何类型的错误”
fn main() {
    //    option_unwrap::test_option_unwrap();
    //    question_mark::test_question_mark();
    //    combinators::test_cominators();
    //    struct_demo::test_struct();

    let value = format!("{:b}", 30).parse::<i64>().unwrap();
    let s = format!("{0:>0width$}", value, width = 8);
    print!("{}", s);
}
