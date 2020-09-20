#[derive(Debug, Clone)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug, Clone)]
struct Peeled(Food);
#[derive(Debug, Clone)]
struct Chopped(Food);
#[derive(Debug, Clone)]
struct Cooked(Food);

mod result;

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}
fn peel2(food: Food) -> Option<Peeled> {
    Some(Peeled(food))
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}
fn chop2(peeled: Peeled) -> Option<Chopped> {
    Some(Chopped(peeled.0))
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}
fn cook2(chopped: Chopped) -> Option<Cooked> {
    Some(Cooked(chopped.0))
}

/// map和then可以传入闭包，函数进行转换，
fn process_question_mark(food: Option<Food>) -> Option<Cooked> {
    cook2(chop2(peel2(food?)?)?)
}

/// map是Option对象的方法，解Option中的值，闭包操作返回的是Option
fn process_map(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

/// and_then是Option对象的方法，解Option中的值，闭包操作的返回是Option中的Some值
fn process_and_then(food: Option<Food>) -> Option<Cooked> {
    food.and_then(|f| Some(Peeled(f)))
        .and_then(|Peeled(f)| Some(Chopped(f)))
        .and_then(|Chopped(f)| Some(Cooked(f)))
}

/// 因为peel1, chop1和cook1版本参数为Option,不能用
fn process_and_then_fn(food: Option<Food>) -> Option<Cooked> {
    food.and_then(peel2).and_then(chop2).and_then(cook2)
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

pub fn test_cominators() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    println!("match=========================================");
    let cooked_apple_match = cook(chop(peel(apple.clone())));
    let cooked_carrot_match = cook(chop(peel(carrot.clone())));
    let cooked_potato_match = cook(chop(peel(potato.clone())));
    eat(cooked_apple_match);
    eat(cooked_carrot_match);
    eat(cooked_potato_match);

    println!("question mark==================================");
    let cooked_apple_question_mark = process_question_mark(apple.clone());
    let cooked_carrot_question_mark = process_question_mark(carrot.clone());
    let cooked_potato_question_mark = process_question_mark(potato.clone());
    eat(cooked_apple_question_mark);
    eat(cooked_carrot_question_mark);
    eat(cooked_potato_question_mark);

    println!("map===========================================");
    let cooked_apple_map = process_map(apple.clone());
    let cooked_carrot_map = process_map(carrot.clone());
    let cooked_potato_map = process_map(potato.clone());
    eat(cooked_apple_map);
    eat(cooked_carrot_map);
    eat(cooked_potato_map);

    println!("and_then========================================");
    let cooked_apple_and_then = process_and_then(apple.clone());
    let cooked_carrot_then = process_and_then(carrot.clone());
    let cooked_potato_and_then = process_and_then(potato.clone());
    eat(cooked_apple_and_then);
    eat(cooked_carrot_then);
    eat(cooked_potato_and_then);

    println!("and_then_fn=======================================");
    let cooked_apple_and_then_fn = process_and_then_fn(apple);
    let cooked_carrot_then_fn = process_and_then_fn(carrot);
    let cooked_potato_and_then_fn = process_and_then_fn(potato);
    eat(cooked_apple_and_then_fn);
    eat(cooked_carrot_then_fn);
    eat(cooked_potato_and_then_fn);

    println!("result_result=======================================");
    result::test_result();
}
