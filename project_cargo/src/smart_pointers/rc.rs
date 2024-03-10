use crate::smart_pointers::rc::List::{Cons, Nil};
use std::{ops::ControlFlow, rc::Rc};

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn example1() {
    // multiple ownership
    let a = Rc::new(Cons(1, Rc::new(Cons(12, Rc::new(Nil)))));
    let b = Cons(12, Rc::clone(&a));
    let c = Cons(14, Rc::clone(&a));

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
}

pub fn example2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
