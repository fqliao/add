use std::cell::Cell;

pub struct Foo {
    pub x: u32,
    pub y: Cell<u32>,
}
