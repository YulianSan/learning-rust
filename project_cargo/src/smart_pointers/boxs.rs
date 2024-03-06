pub fn example1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// recursive enum
// | List                        |
// | i32  | Pointer to other List|
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}
