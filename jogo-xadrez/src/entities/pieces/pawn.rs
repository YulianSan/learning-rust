use crate::entities::pieces::piece_trait::{Eliminable, Movable, Piece, PieceColor, PiecePosition};

pub struct Pawn {
    position: PiecePosition,
    color: PieceColor,
    alive: bool,
}

impl Piece for Pawn {
    fn new(position: PiecePosition, color: PieceColor) -> Self {
        Self { position, color, alive: true }
    }

    fn symbol(&self) -> char {
        match self.color {
            PieceColor::White => '♙',
            PieceColor::Black => '♟',
        }
    }

    fn color(&self) -> &PieceColor {
        &self.color
    }

    fn position(&self) -> &PiecePosition {
        &self.position
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Movable for Pawn {
    fn possible_moves(&self) -> Vec<PiecePosition> {
        match self.color {
            PieceColor::White => {
                vec![PiecePosition{ x: self.position.x, y: self.position.y + 1 }]
            },
            PieceColor::Black => {
                vec![PiecePosition{ x: self.position.x, y: self.position.y - 1 }]
            }
        }
    }

    fn move_to(&mut self, position: PiecePosition) {
        self.position = position;
    }
}

impl Eliminable for Pawn {
    fn eliminate(&mut self) {
        self.alive = false
    }
}
