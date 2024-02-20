use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut count_shirt: HashMap<ShirtColor, i32> = HashMap::new();

        for shirt in &self.shirts {
            *count_shirt.entry(shirt.clone()).or_insert(0) += 1
        }

        // get the max value
        let x = count_shirt
            .iter()
            .max_by_key(|color| color.1)
            .unwrap_or((&ShirtColor::Red, &1));

        x.0.clone()
    }
}

pub fn example1() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
