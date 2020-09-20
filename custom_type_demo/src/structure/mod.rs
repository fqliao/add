#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// a unit struct
#[derive(Debug)]
struct Nil;

// a tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// a struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// struct使用结构体类型
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

/// 1 结构体有三种形式：元组结构体、结构体、单元结构体
pub fn test_structure() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("peter:{:?}", peter);
    print_person(&peter);

    let point1 = Point { x: 0.1, y: 0.2 };
    let point2 = Point { x: 0.3, y: 0.4 };
    println!("point1:({}, {})", point1.x, point1.y);
    // 使用其他结构体数据，便于更新
    let new_point = Point { x: 1.1, ..point1 };
    println!("new_point:{:?}", new_point);

    // 解析结构体
    let Point { x: mx, y: my } = point2;
    println!("mx:{}, my:{}", mx, my);
    let rectangle = Rectangle {
        p1: point1,
        p2: Point { x: mx, y: my },
    };
    println!("rectangle:{:?}", rectangle);

    let unit_struct = Nil;
    println!("{:?}", unit_struct);

    let pair = Pair(1, 1.2);
    println!("pair:({}, {})", pair.0, pair.1);
    // 绑定元组变量
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area:{}", rect_area(&rectangle));
}

fn rect_area(rect: &Rectangle) -> f32 {
    // 嵌套解构
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = rect;
    let area1 = (rect.p2.x - rect.p1.x) * (rect.p2.y - rect.p1.y);
    println!("area1:{}", area1);
    let area = (x2 - x1) * (y2 - y1);
    area
}

fn print_person(person: &Person) {
    // 嵌套解构
    println!("{:?}", person);
}
