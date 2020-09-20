pub fn test_cast() {
    let decimal = 65.123f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!("1000 % 256 is {}", 1000 % 256);
    println!("128 as a i16 {} ", 128 as i16);
    println!("-128 as a i8 {} ", -128 as i8);
}
