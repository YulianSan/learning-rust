use std::char;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

struct Game {
    player_x: usize,
    player_y: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            player_x: WIDTH / 2,
            player_y: HEIGHT / 2,
        }
    }

    fn display(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == self.player_x && y == self.player_y {
                    print!("P ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    fn update(&mut self, input: char) {
        match input {
            'w' => {
                if self.player_y > 0 {
                    self.player_y -= 1
                }
            }
            's' => {
                if self.player_y < HEIGHT - 1 {
                    self.player_y += 1
                }
            }
            'a' => {
                if self.player_x > 0 {
                    self.player_x -= 1
                }
            }
            'd' => {
                if self.player_x < WIDTH - 1 {
                    self.player_x += 1
                }
            }
            _ => (),
        }
    }
}

pub fn main() {
    let mut game = Game::new();

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
        game.display();

        print!("Enter a move (w/a/s/d): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().chars().next();

        if let Some(input) = input {
            game.update(input);
        }

        thread::sleep(Duration::from_millis(100));
    }
}
