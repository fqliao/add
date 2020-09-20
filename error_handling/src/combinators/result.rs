use std::num::ParseIntError;

// 定义Result别名
type ParseIntResult<T> = Result<T, ParseIntError>;

/// 使用match,最好控制错误信息
fn multiply_match(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(v) => v,
        Err(e) => {
            println!("first number str is parsed error");
            return Err(e);
        }
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(v) => v,
        Err(e) => {
            println!("second number str is parsed error");
            return Err(e);
        }
    };
    Ok(first_number * second_number)
}

/// 使用map和and_then
fn multiply_map_and_then(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

/// 使用？,代码更容易阅读，可以看出使用？优于使用map和and_then
fn multiply_question_mark(first_number_str: &str, second_number_str: &str) -> ParseIntResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: ParseIntResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn test_result() {
    let twenty = multiply_match("10", "2");
    print(twenty);
    let tt = multiply_match("2", "t");
    print(tt);

    let twenty = multiply_map_and_then("10", "2");
    print(twenty);
    let tt = multiply_map_and_then("t", "2");
    print(tt);

    let twenty = multiply_question_mark("10", "2");
    print(twenty);
    let tt = multiply_question_mark("t", "2");
    print(tt);
}
