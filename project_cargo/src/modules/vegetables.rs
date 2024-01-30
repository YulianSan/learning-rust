#[derive(Debug)]
pub struct Asparagus {
    pub name: String,
    pub color: String,
}

pub fn show_vegetable(vegetable: Asparagus) {
    println!("{:?}", vegetable);
}
