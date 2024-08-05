extern crate termion;

use crate::entities::pieces::{
    pawn::Pawn,
    piece_trait::{Piece, PieceColor, PiecePosition},
};

pub enum GameType {
    Normal,
}

pub struct Board {
    pub rows: i32,
    pub columns: i32,
    pub pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new(game_type: GameType) -> Self {
        match game_type {
            GameType::Normal => Self::normal_game(),
        }
    }

    pub fn normal_game() -> Self {
        Self {
            rows: 8,
            columns: 8,
            pieces: vec![Box::new(Pawn::new(
                PiecePosition { x: 1, y: 1 },
                PieceColor::White,
            ))],
        }
    }

    pub fn get_piece(&self, x: i32, y: i32) -> Option<char> {
        *&self
            .pieces
            .iter()
            .find(|&piece| piece.position().x == x && piece.position().y == y)
            .and_then(|piece| Some(piece.symbol()))
    }

    pub fn draw(&self) -> Vec<Vec<Option<char>>> {
        let mut draw_vec: Vec<Vec<Option<char>>> = vec![];
        for y in (1..=self.rows).rev() {
            println!("{y}");
            let mut draw_row: Vec<Option<char>> = vec![];
            draw_row.push(y.to_string().chars().next());
            draw_row.push(None);

            for x in 1..=self.columns {
                draw_row.push(self.get_piece(x, y));
            }
            draw_vec.push(draw_row);

            if y == 1 {
                draw_vec.push(
                    vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
                        .iter()
                        .map(|i| Some(*i))
                        .collect(),
                );

                draw_vec.push(
                    vec![' ', ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
                        .iter()
                        .map(|i| Some(*i))
                        .collect(),
                );
            }
        }

        draw_vec
    }
}
