use ch01::create_function;
use ch01::lexical;
use ch01::trait_demo::S;
use ch01::trait_demo::{T1, T2};

fn main() {
    lexical::macro_show();
    // Create functions named `foo` and `bar` with the above macro.
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    // S::f()  // Calls the inherent impl.
    // 完全限定无歧义调用
    <S as T1>::f(); // Calls the T1 trait function.
    <S as T2>::f(); // Calls the T2 trait function.

    let vec1 = (0..10).collect::<Vec<_>>();
    let vec2 = Vec::<u8>::with_capacity(1024);
    println!("vec1:{:?}", vec1);
    println!("vec2:{:?}", vec2);
}
