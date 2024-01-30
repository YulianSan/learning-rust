pub mod food;
pub mod vegetables;

// private because no have pub
mod test {
    pub fn test() {
        println!("Test in mod test");
        // call test outside mod
        // super is used to call parent mod
        super::test();
    }
}

pub fn test() {
    println!("Test in fn");
}
