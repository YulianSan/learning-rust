fn main() {
    let mut x = 0;
    x = 2;
    increment(&mut x.clone());
    println!("x: {x}");
}

fn increment(x: &mut i32) {
    *x += 1;
}
