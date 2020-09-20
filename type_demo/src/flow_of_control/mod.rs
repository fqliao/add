use crate::flow_of_control::Brz::Qux;

/// 1 条件控制语句是一个表达式
/// 2 布尔语句不需要括号
/// 3 条件分支必须返回相同类型的值

pub fn test_if_else() {
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };
    println!("{} -> {}", n, big_n);
}

/// loop默认无限循环
/// continue:退出本次迭代
/// break:退出loop循环
pub fn test_loop() {
    let mut count = 0_u32;
    println!("Let's count until infinity!");
    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("Ok, this is enough.");
            // Exit this loop
            break;
        }
    }
}

/// 1 定义label可以从内部loop跳出外部loop
pub fn test_loop_nest_label() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            // This would break only the inner loop
            //            break;
            // This would break the outer loop
            break 'outer;
        }
        println!("Returned the outer loop");
        // This would break the outer loop
        break;
    }
    println!("Exited the outer loop");
}

/// 1 loop是个表达式
/// 2 获取loop的表达式的值，使用break返回
pub fn test_loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result:{}", result);
}

/// 1 while true = loop
pub fn test_while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("buzz");
        }
        n += 1;
    }
}

/// 1 for i in a..b形式，迭代范围[a..b)
/// 2 for i in a..=b形式，迭代范围[a..b]
pub fn test_for_in() {
    let mut n = 1;
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("buzz");
        }
    }
}

/// 1 遍历vec: in 0..vec.len() 或 in vec.iter()，不会移动vec，因此遍历后可用
/// 2 match 比if else还省代码
pub fn test_for_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    //    for i in 0..names.len(){
    //        match names[i] {
    //            "Ferris" => println!("There is a rustacean among us!"),
    //            _ => println!("Hello {}", names[i]),
    //        }
    //    }
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("vec:{:?}", names);
}

/// 遍历vec: vec.into_iter(), 会移动vec，因此遍历后不可用
pub fn test_for_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // useless vec
    // println!("vec:{:?}", names);
}

/// 遍历vec: vec.iter_mut(), 可修改vec的元素，vec不会移动vec，因此遍历后可用
/// 注意：mut 是类型的一部分
pub fn test_for_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    //    for i in 0..names.len() {
    //        if names[i] == "Ferris" {
    //            names[i] = "There is a rustacean among us!";
    //        } else {
    //            names[i] = "Hello";
    //        }
    //    }
    println!("vec:{:?}", names);
}

/// 1 match 类似于C中的switch,但比swich强大很多
/// 2 match 也是表达式，可以赋值，也是强大的表现
pub fn test_match() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 9 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"),
        _ => println!("Ain' t special"),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("{} -> {}", boolean, binary);
}

/// 1 match 匹配pair
pub fn test_match_tuple() {
    let pair = (0, 2);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("y:{}", y),
        (x, 0) => println!("x:{}", x),
        _ => println!("x and y is not 0"),
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
/// 1 match 匹配enum
pub fn test_match_enum() {
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
}

/// 1 match 匹配引用
/// 2 ref与&的区别：
/// 可以主动将匹配自转为引用
/// & 只能匹配原值是否为引用
/// 3 修改匹配后的值，需要声明ref mut进行匹配
pub fn test_match_ref() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            // 这里加ref mut才会生效，也就可以改变m的值
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}
/// 1 match struct
pub fn test_match_struct() {
    let foo = Foo { x: (1, 3), y: 2 };

    match foo {
        //        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y);
    }
}

/// 1 match + if结合匹配
pub fn test_match_guards() {
    let pair = (3, 0);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn age() -> u32 {
    3
}
fn some_number() -> Option<u32> {
    Some(18)
}
/// 1 @可以绑定匹配范围内的值
/// 如果直接a...b则知道匹配了，但不知道具体值
pub fn test_match_blinding() {
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        Some(0) => println!("I'm not born yet I guess"),
        Some(n @ 1..=12) => println!("I'm a child of age {:?}", n),
        Some(n @ 13..=19) => println!("I'm a teen of age {:?}", n),
        Some(n) => println!("I'm an old person of age {:?}", n),
        None => println!("I'm not born yet I guess too"),
    }
}

/// 1 if let 适合Option Result, Enum, 支持else if和else, 简化match语法
/// 2 if let 匹配使用=
/// 3 if let 不像match严格，要全部穷举，可以不用else
pub fn test_if_let_option() {
    let optional = Some(7);
    match optional {
        Some(n) => println!("This is a really long string and `{:?}`", n),
        _ => {}
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letter = false;
    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

//#[derive(PartialEq)]
enum Brz {
    Bar,
    Baz,
    Qux(u32), // 元组结构体
}

/// 1 if let enum也可以用@绑定枚举中的内部元素
/// 2 if let 比直接使用枚举==更好用，因为不加#[derive(PartialEq)]和实现PartialEq均可使用if let，此时不能用==
pub fn test_if_let_enum() {
    let a = Brz::Bar;
    let b = Brz::Baz;
    let c = Qux(100);

    if let Brz::Bar = a {
        println!("a is foobar");
    }
    if let Brz::Bar = b {
        println!("b is foobar");
    }
    if let Brz::Qux(value) = c {
        println!("c is {}", value);
    }
    if let Brz::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // Error if not add #[derive(PartialEq)]
    //    if Brz::Bar == a {
    //        println!("a is foobar");
    //    }
}

/// while let 适合用于Option, Result, Enum类型判断循环条件
pub fn test_while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("{}", i);
            optional = None;
        } else {
            println!("{}", i);
            optional = Some(i + 1);
        }
    }
}
/// 不用while let的实现方式，loop+match
pub fn test_loop_option() {
    let mut optional = Some(0);
    loop {
        let i = match optional {
            Some(v) => v,
            None => break,
        };
        if i > 9 {
            println!("{}", i);
            optional = None;
        } else {
            println!("{}", i);
            optional = Some(i + 1);
        }
    }
}
