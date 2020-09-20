enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(key) => println!("key {} press", key),
        WebEvent::Paste(v) => println!("paste {}", v),
        WebEvent::Click { x, y } => println!("click ({}, {})", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

/// 1 枚举可以放有限个各种类型数据，非常强大
/// 2 枚举也可以实现方法
/// 3 可以给枚举其别名，针对枚举名称很长的情况
pub fn test_enum() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let add = Operations::Add;
    let subtract = Operations::Subtract;
    println!("result1:{}", add.run(1, 2));
    println!("result2:{}", subtract.run(1, 2));
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

/// 使用枚举可以使用use引其元素
pub fn test_use() {
    use crate::enum_demo::Status::{Poor, Rich};
    use crate::enum_demo::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Poor => println!("poor"),
        Rich => println!("rich"),
    }
    match work {
        Civilian => println!("civilian"),
        Soldier => println!("soldier"),
    }
}

// 序号从0开始
enum Number {
    Zero,
    One,
    Two,
}
/// 注意：给枚举元素设置值，对于枚举元素包含参数，则不支持设置值
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

/// 枚举可以像c风格那样获取序号
pub fn test_c_like() {
    // 将枚举元素转为整数
    println!("Number::Zero is {}", Number::Zero as i32);
    println!("Number::One is {}", Number::One as i32);
    println!("Color::Red is 0x{:06x}", Color::Red as i32);
    println!("Color::Green is 0x{:06x}", Color::Green as i32);
    println!("Color::Blue is 0x{:06x}", Color::Blue as i32);
}

/// 枚举适合做链表
enum List {
    Cons(u32, Box<List>),
    Nil,
}
impl List {
    fn new() -> Self {
        List::Nil
    }
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}
pub fn test_link_list() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("len:{}", list.len());
    println!("list:{}", list.stringify());
}
