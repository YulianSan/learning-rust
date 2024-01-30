#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub color: String,
}

pub fn show_food(food: Food) {
    println!("{:?}", food);
}
