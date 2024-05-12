use std::collections::BTreeMap;
use std::fmt::Debug;
use shakmaty::{Chess, Move, Position};
use shakmaty::san::{San, SanError};
use crate::MoveNumber;

#[derive(Debug, Clone)]
pub struct SanErrorWithMoveNumber(pub SanError, pub MoveNumber);

/// An always legal variation.
/// Internals are private to avoid corrupting the variation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variation {
    moves: Vec<(Move, Chess)>,
    variations: BTreeMap<MoveNumber, Self>,
    move_number: MoveNumber,
    starting_position: Chess,
}

impl Variation {
    /// Use [`Self::new_variation_at`] for subvariations.
    pub fn new_root_variation(move_number: MoveNumber, starting_position: Chess, moves_capacity: usize) -> Self {
        Self {
            moves: Vec::with_capacity(moves_capacity),
            variations: BTreeMap::new(),
            move_number,
            starting_position,
        }
    }
    
    /// Creates a new root variation with the starting move number (white to move - 1) and starting position.
    pub fn new_starting_root_variation() -> Self {
        Self::new_root_variation(MoveNumber::MIN, Chess::new(), 100)
    }
    
    /// Returns [`None`] if the `move_number` argument doesn't match any moves in the variation.
    pub fn new_variation_at(&self, move_number: MoveNumber, moves_capacity: usize) -> Option<Self> {
        if move_number.index == 0 {
            Some(Self::new_root_variation(move_number, self.starting_position.clone(), moves_capacity))
        } else {
            // CLIPPY: If the arithmetic and as conversions return some bogus, then `.get` will return `None` and all is well.
            #[allow(clippy::arithmetic_side_effects)]
            #[allow(clippy::as_conversions)]
            Some(Self::new_root_variation(move_number, self.moves.get((move_number.index - self.move_number.index - 1) as usize)?.1.clone(), moves_capacity))
        }
    }

    #[must_use]
    /// Creates a variation based on the position *before* the last move was played.
    pub fn new_variation_at_last_move(&self, moves_capacity: usize) -> Self {
        self.last_move_number()
            .index
            .checked_sub(1)
            .map_or_else(|| Self::new_root_variation(MoveNumber { index: 0 }, self.starting_position.clone(), moves_capacity),
                         |last_move_number_sub1|
                             self.moves
                                 .len()
                                 .checked_sub(2)
                                 .map_or_else(||
                                                  Self::new_root_variation(MoveNumber { index: last_move_number_sub1 }, self.starting_position.clone(), moves_capacity),
                                              |moves_len_sub2|
                                                  Self::new_root_variation(MoveNumber { index: last_move_number_sub1 }, self.moves.get(moves_len_sub2).map_or(self.starting_position.clone(), |m| m.1.clone()), moves_capacity)))
    }

    /// A vector of moves and the position that occurs in the variation *after* the move is played.
    pub const fn moves(&self) -> &Vec<(Move, Chess)> {
        &self.moves
    }

    pub const fn variations(&self) -> &BTreeMap<MoveNumber, Self> {
        &self.variations
    }

    /// The position before any move in the variation was played.
    pub const fn starting_position(&self) -> &Chess {
        &self.starting_position
    }

    pub fn last_position(&self) -> &Chess {
        self.moves.last().map_or(&self.starting_position, |m| &m.1)
    }

    /// The `move_number` of the first move in the variation.
    pub const fn move_number(&self) -> MoveNumber {
        self.move_number
    }
    
    pub fn last_move_number(&self) -> MoveNumber {
        // CLIPPY: There's never going to be so many moves that this overflows or truncates.
        #[allow(clippy::arithmetic_side_effects)]
        #[allow(clippy::as_conversions)]
        #[allow(clippy::cast_possible_truncation)]
        {
            MoveNumber { index: self.move_number.index + self.moves.len() as u16 }
        }
    }

    /// Like [`Vec::push`], but checks if the new move is valid.
    /// 
    /// # Errors
    /// 
    /// The given SAN is an illegal move.
    pub fn push_move(&mut self, san: &San) -> Result<(), SanError> {
        let end_position = self.moves.last().map_or(&self.starting_position, |m| &m.1);
        
        match san.to_move(end_position) {
            Ok(r#move) => {
                let mut end_position = end_position.clone();
                end_position.play_unchecked(&r#move);
                self.moves.push((r#move, end_position));
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    /// Removing the last move is the only option, because removing a move in the middle would invalidate the moves after.
    pub fn pop_move(&mut self) -> Option<(Move, Chess)> {
        self.moves.pop()
    }

    pub fn insert_variation(&mut self, variation: Self) -> Option<Self> {
        self.variations.insert(variation.move_number, variation)
    }

    pub fn remove_variation(&mut self, move_number: MoveNumber) -> Option<Self> {
        self.variations.remove(&move_number)
    }

    pub fn get_variation(&self, move_number: MoveNumber) -> Option<&Self> {
        self.variations.get(&move_number)
    }
    
    pub fn move_at(&self, move_number: MoveNumber) -> Option<&(Move, Chess)> {
        // CLIPPY: If this arithmetic returns something bogus then `.get` will return `None` and all is well.
        #[allow(clippy::arithmetic_side_effects)]
        #[allow(clippy::as_conversions)]
        {
            self.moves.get((move_number.index - self.move_number.index) as usize)
        }
    }
}

/// Pushes the given moves to the variation, returning the first error.
///
/// Syntax: `push_moves!(variation, move1, move2)`.
#[macro_export]
macro_rules! push_moves {
    ($variation:expr, $($r#move:expr),*) => {
        {
            fn push_moves(variation: &mut Variation) -> Result<(), SanError> {
                $(
                variation.push_move($r#move)?;
                )*
                
                Ok(())
            }
            
            push_moves(&mut $variation)
        }
    };
}

// This is used in tests.
#[allow(unused_imports)]
pub(crate) use push_moves;
