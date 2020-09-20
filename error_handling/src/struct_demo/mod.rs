#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    gender: &'a str,
    age: i32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

pub fn test_struct() {
    let name = "alice";
    let gender = "女";
    let age = 12;
    let mut alice = Person { name, gender, age };
    println!("alice:{:#?}", alice);
    println!("name:{:#?}", alice.name);
    println!("gender:{:#?}", alice.gender);
    println!("age:{:#?}", alice.age);
    alice.age = 18;
    println!("age:{:#?}", alice.age);

    let bob = Person {
        name: "bob",
        gender: "男",
        ..alice
    };
    println!("bob:{:#?}", bob);

    let color = Color(1, 2, 3);
    println!("color:{:#?}", color);
    println!("color:{:#?}", color.0);
    println!("color:{:#?}", color.1);
    println!("color:{:#?}", color.2);
}
