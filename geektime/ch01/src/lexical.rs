
pub fn macro_show() {
  macro_rules! calculate {
    ($e:expr) => {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
    };
  }
  calculate!(1 + 2 + 7);
  calculate!((1 + 2) * (8 / 4));
}