use shakmaty::{Chess, Move, PlayError, Position};
use shakmaty::san::{San, SanError};
use crate::Variation;

/// A position with a move that was played on it and a list of variations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turn {
    position: Chess,
    played_move: Move,
    variations: Vec<Variation>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoSuchTailTurn(pub usize);

impl Turn {
    pub fn new(position: Chess, move_played: Move, variations_capacity: usize) -> Self {
        Self {
            position,
            played_move: move_played,
            variations: Vec::with_capacity(variations_capacity)
        }
    }

    pub fn from_san(position: Chess, move_played: &San, variations_capacity: usize) -> Result<Self, SanError> {
        let move_played = move_played.to_move(&position)?;

        Ok(Self {
            position,
            played_move: move_played,
            variations: Vec::with_capacity(variations_capacity)
        })
    }

    /// Creates a new [`Turn`] with a `variations` capacity of 2.
    pub fn from_san_with_default_capacity(position: Chess, move_played: &San) -> Result<Self, SanError> {
        let move_played = move_played.to_move(&position)?;

        Ok(Self {
            position,
            played_move: move_played,
            variations: Vec::with_capacity(2)
        })
    }

    /// Creates a new [`Turn`] with a `variations` capacity of 2.
    pub fn with_default_capacity(position: Chess, move_played: Move) -> Self {
        Self {
            position,
            played_move: move_played,
            variations: Vec::with_capacity(2)
        }
    }

    pub fn position(&self) -> &Chess {
        &self.position
    }

    pub fn move_played(&self) -> &Move {
        &self.played_move
    }

    pub fn variations(&self) -> &Vec<Variation> {
        &self.variations
    }

    /// Pushes this to the list of this turn's variation, if it can be played on this turn's position.
    ///
    /// # Panics
    ///
    /// Panics if the new variation vector capacity exceeds `isize::MAX` bytes.
    pub fn push_variation(&mut self, variation: Variation) -> Result<(), PlayError<Chess>> {
        &self.position.clone().play(variation.first_turn().move_played())?;

        self.variations.push(variation);

        Ok(())
    }

    /// Equivalent to [`Vec::remove`].
    pub fn remove_variation(&mut self, index: usize) -> Variation {
        self.variations.remove(index)
    }
}