use std::{collections::HashMap, thread, time::Duration};

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

pub fn example2() {
    let expensive_closure = |num: u32| -> u32 {
        let seconds = Duration::from_secs(num as u64);

        println!("sleep {:?}", seconds);
        thread::sleep(seconds);
        num
    };

    expensive_closure(2);
    println!("finish");
}

pub fn example3() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // // error because params x is String because line above
    // let n = example_closure(5);
}

pub fn example4() {
    let mut list = vec![];
    let x: String = String::from("hello");
    println!("Before defining closure: {:?}", list);

    // FnOnce
    let thread_closure = move || {
        // x is moved
        list.push(x);
        println!("From thread: {:?}", list);
    };

    // only can call once
    // thread_closure();

    // taking ownership of variable test
    thread::spawn(thread_closure).join().unwrap();
}

pub fn example5() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 9,
            height: 12,
        },
    ];

    let mut count = 0;

    list.sort_by_key(|item| {
        count += 1;
        println!("a: {:?}, count: {}", item, count);
        item.width
    });
}
