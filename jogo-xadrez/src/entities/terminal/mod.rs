use std::{
    io::{stdin, Read, Write},
    ops::Deref,
};

use termion::{
    clear, color, cursor,
    event::{Event, Key},
    input::TermRead,
    style::{self, Blink},
};

extern crate termion;

pub struct Terminal<R, W: Write> {
    pub stdout: W,
    pub stdin: R,
}

impl<R, W: Write> Terminal<R, W> {
    pub fn new(stdin: R, stdout: W) -> Self {
        Self { stdin, stdout }
    }

    pub fn draw_vec(&mut self, vec: Vec<Vec<Option<char>>>) {
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(3, 3)
        )
        .unwrap();

        for (i_row, row) in vec.iter().enumerate() {
            for (i_col, col) in row.iter().enumerate() {
                if i_col > 1 && i_row < 8 {
                    let index = i_col + (i_row % 2);
                    write!(
                        self.stdout,
                        "{}{}{}",
                        color::Bg(self.get_color(index as u16).deref()),
                        color::Fg(color::LightYellow),
                        col.unwrap_or(' ')
                    )
                    .unwrap();
                } else {
                    write!(
                        self.stdout,
                        "{}{}{}",
                        color::Bg(color::Rgb(150, 75, 0)),
                        color::Fg(color::LightWhite),
                        col.unwrap_or(' ')
                    )
                    .unwrap();
                }
            }
            write!(self.stdout, "{}", cursor::Goto(3, 4 + i_row as u16)).unwrap();
        }
    }

    pub fn get_color(&self, i: u16) -> Box<dyn color::Color> {
        if i % 2 == 0 {
            Box::new(color::LightWhite)
        } else {
            Box::new(color::Black)
        }
    }

    pub fn read_line(&mut self) {
        self.stdout.flush().unwrap();
        let intstd = stdin();
        for c in intstd.events() {
            match c {
                Ok(key) => match key {
                    Event::Key(Key::Ctrl('c')) => {
                        break;
                    }
                    Event::Key(Key::Left) => {
                        write!(
                            self.stdout,
                            "{}{}{} {}",
                            cursor::Save,
                            cursor::Goto(4, 4),
                            color::Bg(color::Blue),
                            cursor::Restore
                        )
                        .unwrap();
                        self.stdout.flush().unwrap();
                    }
                    _ => {}
                },
                Err(_) => {}
            }
        }
    }
}
