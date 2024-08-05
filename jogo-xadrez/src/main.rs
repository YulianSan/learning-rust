extern crate termion;

use entities::terminal::Terminal;
use std::io::{self, stdin, Read, Write};
use termion::raw::IntoRawMode;

mod entities;
fn main() {
    let board = entities::board::board::Board::new(entities::board::board::GameType::Normal);

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdout = stdout.into_raw_mode().unwrap();

    let mut terminal = Terminal::new(io::stdin(), stdout);
    terminal.draw_vec(board.draw());
    terminal.read_line();
}
