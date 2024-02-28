use add_one::add_one;
// this cause a error because no is added in Cargo.toml
// the version of rand must be 0.8.5
// use rand;

fn main() {
    println!("Hello from adder!, Result from add_one: {}", add_one(1));
}
