use std::{cell::RefCell, rc::Rc};

pub fn example1() {
    // RefCell ignore borrowing rules
    let x = Rc::new(RefCell::new(10));
    let y = Rc::clone(&x);
    let z = (Rc::clone(&x), 10);
    // change value immutable
    *x.borrow_mut() += 1;

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
    println!("{:?}", Rc::strong_count(&x));
}
