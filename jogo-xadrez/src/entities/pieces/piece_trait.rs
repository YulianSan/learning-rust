pub struct PiecePosition {
    pub x: i32,
    pub y: i32,
}

pub enum PieceColor {
    White,
    Black,
}

pub trait Movable {
    fn possible_moves(&self) -> Vec<PiecePosition>;
    fn move_to(&mut self, position: PiecePosition);
}

pub trait Eliminable {
    fn eliminate(&mut self);
}

pub trait Piece {
    fn new(position: PiecePosition, color: PieceColor) -> Self where Self: Sized;
    fn symbol(&self) -> char;
    fn color(&self) -> &PieceColor;
    fn position(&self) -> &PiecePosition;
    fn alive(&self) -> bool;
}
