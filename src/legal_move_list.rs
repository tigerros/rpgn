use shakmaty::{Chess, Move, Position};

/// An always legal list of moves.
pub struct LegalMoveList {
    last_position: Chess,
    moves: Vec<Move>,
}

impl LegalMoveList {
    pub fn new(last_position: Chess, move_capacity: usize) -> Self {
        Self {
            last_position,
            moves: Vec::with_capacity(move_capacity),
        }
    }
    
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
    
    pub fn last_position(&self) -> &Chess {
        &self.last_position
    }

    pub fn play(&mut self, r#move: Move) -> Result<(), ()> {
        if !self.last_position.is_legal(&r#move) {
            return Err(());
        }

        self.last_position.play_unchecked(&r#move);
        self.moves.push(r#move);

        Ok(())
    }
}