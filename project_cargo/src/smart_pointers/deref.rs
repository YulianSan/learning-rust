use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// need Deref trait
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn example1() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    // no work because y is a pointer
    // assert_eq!(y, 5);
    assert_eq!(*y, 5);
    assert_eq!(y, &5);
}

pub fn example2() {
    let x = 5;
    let my_box = MyBox::new(5);

    assert_eq!(x, *my_box);
}

pub fn example3() {
    let my_box_string = MyBox::new(String::from("hello"));

    fn hello(x: &str) {
        println!("Hello {x}")
    }

    // same result
    hello(&(*my_box_string)[..]);
    hello(&my_box_string);
}
