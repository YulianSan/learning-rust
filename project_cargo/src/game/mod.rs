use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn game2() {
    let x = Guess::new(100);
    println!("{}", x.value());
}

pub fn game() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess.trim());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!();
    }
}
