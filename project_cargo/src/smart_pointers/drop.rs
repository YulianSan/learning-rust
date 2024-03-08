#[derive(Debug)]
struct CustomSmartPointerDrop {
    data: String,
}

impl CustomSmartPointerDrop {
    fn new(data: String) -> CustomSmartPointerDrop {
        CustomSmartPointerDrop { data }
    }
}

impl Drop for CustomSmartPointerDrop {
    // call when variable out of scope
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}
pub fn example1() {
    let x: CustomSmartPointerDrop = CustomSmartPointerDrop::new(String::from("hello"));
    let y: CustomSmartPointerDrop = CustomSmartPointerDrop::new(String::from("world"));

    println!("x and y is created");
}

pub fn example2() {
    let x: CustomSmartPointerDrop = CustomSmartPointerDrop::new(String::from("hello"));

    // can't make this
    // x.drop();
    drop(x);
    // no make this
    // println!("x is dropped {:?}", x);
}
