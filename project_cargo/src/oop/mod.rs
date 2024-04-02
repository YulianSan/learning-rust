use std::rc::Rc;

pub mod encapsulation;
pub mod gui;
pub mod patterns;
pub mod post;

fn example1() {
    trait Test1 {
        fn new() -> Self
        where
            Self: Sized;
    }

    // dyn no have lenght, that's why need a pointer to say the address and lenght
    let x: Vec<Box<dyn Test1>> = vec![];
    let x1: Vec<&dyn Test1> = vec![];
    let x1: Vec<Rc<&dyn Test1>> = vec![];
}
