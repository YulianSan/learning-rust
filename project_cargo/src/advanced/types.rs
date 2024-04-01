pub fn example1() {
    // type is equal typescript, prevent repeat many times
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!());

    fn takes_long_type(f: Thunk) {
        ()
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| {})
    }
}

// type never, equal typescript
pub fn example2() -> ! {
    panic!("type never")
}

pub fn example3() {
    // str is a type and need know how much memory to allocate
    // &str is two values: address and lenght
    /*
        the same thing:
        &str, Box<str> or Rc<str>
        &dyn Trait, Box<dyn Trait> or Rc<dyn Trait>
    */
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
}
