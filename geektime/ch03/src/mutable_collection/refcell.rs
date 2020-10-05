use std::cell::RefCell;

pub fn test_refcell() {
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    println!("{:?}", ref_cell.borrow());
    ref_cell.borrow_mut().push(4);
    println!("{:?}", ref_cell.borrow());
}
