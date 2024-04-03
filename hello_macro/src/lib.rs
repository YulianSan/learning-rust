use hello_macro_derive::HelloMacro;

// need define trait to macro
pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

pub fn main() {
    Pancakes::hello_macro();
}
