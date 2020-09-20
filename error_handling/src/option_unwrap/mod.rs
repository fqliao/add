/// All gifts are handled explicitly using `match`.
pub fn give_commoner_match(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(v) => println!("{}? How nice.", v),
        None => println!("No gift? Oh well."),
    }
}

/// All gifts are handled explicitly using `unwrap`.
/// unwrap used to test
pub fn give_commoner_unwrap(gift: Option<&str>) {
    let gift = gift.unwrap();
    if gift == "snake" {
        println!("AAAAaaaa!");
    } else {
        println!("{}? How nice.", gift);
    }
}

pub fn test_option_unwrap() {
    let snake = Some("snake");
    let food = Some("cabbage");
    let nothing = None;
    give_commoner_match(snake);
    give_commoner_match(food);
    give_commoner_match(nothing);
    give_commoner_unwrap(snake);
    give_commoner_unwrap(food);
    //  give_commoner_unwrap(nothing);
}
