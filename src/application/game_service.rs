use crate::domain::board::{Board, MoveEvent};
use crate::domain::direction::Direction;

pub struct GameService {
    board: Board,
}

impl GameService {
    pub fn new(size: usize) -> Self {
        Self { board: Board::new(size) }
    }

    pub fn board(&self) -> &Board { &self.board }

    pub fn is_over(&self) -> bool { !self.board.can_move() }

    pub fn is_won(&self) -> bool { self.board.is_won() }

    pub fn score(&self) -> u32 { self.board.score }

    pub fn reset(&mut self) {
        let size = self.board.size;
        self.board = Board::new(size);
    }

    pub fn slide(&mut self, dir: Direction) -> bool {
        self.board.slide(dir)
    }

    pub fn slide_with_events(&mut self, dir: Direction) -> (bool, Vec<MoveEvent>) {
        self.board.slide_with_animations(dir)
    }
}

