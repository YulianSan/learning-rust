use rand::Rng;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            println!("Success")
        }
        Err(error) => {
            println!("Failed to read line {}", error.kind())
        }
    }

    println!("You guessed: {}", guess);

    if secret_number == guess.trim().parse().expect("Not a number") {
        println!("You win!");
    }
}
