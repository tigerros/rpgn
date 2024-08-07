use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Deref;
use shakmaty::{Chess, Move, PlayError, Position};
use shakmaty::san::{San, SanError};

#[derive(Debug, Clone)]
pub struct VariationsCapacity(pub usize);

impl Default for VariationsCapacity {
    /// 3
    fn default() -> Self {
        Self(3)
    }
}

/// A move that was played and a list of variations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turn {
    played_move: Move,
    variations: Vec<Variation>
}

impl Turn {
    pub fn new(move_played: Move, variations_capacity: VariationsCapacity) -> Self {
        Self {
            played_move: move_played,
            variations: Vec::with_capacity(variations_capacity.0)
        }
    }

    pub fn played_move(&self) -> &Move {
        &self.played_move
    }

    pub fn variations(&self) -> &Vec<Variation> {
        &self.variations
    }

    pub fn get_variation_mut(&mut self, index: usize) -> Option<&mut Variation> {
        self.variations.get_mut(index)
    }
}

/// An always legal variation with a history of [`Turn`]s.
/// 
/// Internally, it is composed of a first position and first turn fields, and then a tail of turns vector.
/// However, all functions that accept an "index" as a parameter will treat the first turn and the tail as one list. 
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variation {
    first_position: Chess,
    first_turn: Turn,
    tail_turns: Vec<Turn>
}

#[derive(Debug)]
pub enum InsertVariationError {
    NoSuchTurn { index: usize },
    /// The position at the specified index does not match the new variation's starting position.
    PositionDoesNotMatch
}

#[derive(Debug)]
pub enum PlayAtError {
    NoSuchTurn { index: usize },
    PlayError(VariationPlayError)
}

/// The position does not match the new variation's starting position.
#[derive(Debug, Clone, Copy)]
pub struct PositionDoesNotMatchError;

/// This is equivalent to [`shakmaty::PlayError`], but with public fields.
#[derive(Debug, Clone)]
pub struct PlayError2<P> where P: Position {
    pub position: P,
    pub r#move: Move
}

#[derive(Debug, Copy, Clone)]
pub struct NoSuchTurnError {
    pub index: usize
}

#[derive(Debug)]
pub struct VariationPlayError {
    pub at_position: usize,
    pub r#move: Move,
}

#[derive(Debug, Clone)]
pub struct FirstPosition(pub Chess);

impl Default for FirstPosition {
    /// [`Chess::new()`]
    fn default() -> Self {
        Self(Chess::new())
    }
}

#[derive(Debug, Clone)]
pub struct TailTurnsCapacity(pub usize);

impl Default for TailTurnsCapacity {
    /// 100
    fn default() -> Self {
        Self(100)
    }
}

impl Variation {
    /// Errors if the `first_turn` is not legal in the `first_position`.
    pub fn new(first_position: FirstPosition, first_turn: Turn, tail_turns_capacity: TailTurnsCapacity) -> Result<Self, PlayError2<Chess>> {
        let first_position = first_position.0;

        if first_position.is_legal(&first_turn.played_move) {
            Ok(Self {
                first_position,
                first_turn,
                tail_turns: Vec::with_capacity(tail_turns_capacity.0),
            })
        } else {
            Err(PlayError2 {
                position: first_position,
                r#move: first_turn.played_move
            })
        }
    }

    /// Errors if the `first_move` is not legal in the `first_position`.
    ///
    ///
    pub fn from_move(first_position: FirstPosition, first_move: Move, tail_turns_capacity: TailTurnsCapacity, variations_capacity: VariationsCapacity) -> Result<Self, PlayError2<Chess>> {
        let first_position = first_position.0;

        if first_position.is_legal(&first_move) {
            Ok(Self {
                first_position,
                first_turn: Turn::new(first_move, variations_capacity),
                tail_turns: Vec::with_capacity(tail_turns_capacity.0),
            })
        } else {
            Err(PlayError2 {
                position: first_position,
                r#move: first_move
            })
        }
    }

    /// Errors if the `first_move` is not legal in the `first_position`.
    ///
    ///
    pub fn from_san(first_position: FirstPosition, first_move: San, tail_turns_capacity: TailTurnsCapacity, variations_capacity: VariationsCapacity) -> Result<Self, SanError> {
        let first_position = first_position.0;
        let first_move = first_move.to_move(&first_position)?;

        Ok(Self {
            first_position,
            first_turn: Turn::new(first_move, variations_capacity),
            tail_turns: Vec::with_capacity(tail_turns_capacity.0),
        })
    }

    pub fn first_position(&self) -> &Chess {
        &self.first_position
    }

    pub fn first_turn(&self) -> &Turn {
        &self.first_turn
    }

    pub fn first_turn_mut(&mut self) -> &mut Turn {
        &mut self.first_turn
    }

    pub fn tail_turns(&self) -> &Vec<Turn> {
        &self.tail_turns
    }

    pub fn last_position(&self) -> Cow<Chess> {
        if self.tail_turns.is_empty() {
            return Cow::Borrowed(&self.first_position);
        }

        let mut last_position = self.first_position.clone();

        for turn in &self.tail_turns {
            last_position.play_unchecked(&turn.played_move);
        }

        Cow::Owned(last_position)
    }
    
    pub fn get_position(&self, index: usize) -> Option<Cow<Chess>> {
        if index == 0 {
            return Some(Cow::Borrowed(&self.first_position));
        } else if index > self.tail_turns.len() {
            return None;
        }

        let mut requested_position = self.first_position.clone();
        let mut turns_traversed = 0;

        for turn in &self.tail_turns {
            if index == turns_traversed {
                break;
            }

            requested_position.play_unchecked(&turn.played_move);

            turns_traversed += 1;
        }

        Some(Cow::Owned(requested_position))
    }

    pub fn last_turn(&self) -> &Turn {
        if let Some(last_turn) = self.tail_turns.last() {
            last_turn
        } else {
            &self.first_turn
        }
    }

    pub fn last_turn_mut(&mut self) -> &mut Turn {
        if let Some(last_turn) = self.tail_turns.last_mut() {
            last_turn
        } else {
            &mut self.first_turn
        }
    }

    pub fn get_turn(&self, index: usize) -> Option<&Turn> {
        if index == 0 {
            Some(&self.first_turn)
        } else {
            self.tail_turns.get(index - 1)
        }
    }

    pub fn get_turn_mut(&mut self, index: usize) -> Option<&mut Turn> {
        if index == 0 {
            Some(&mut self.first_turn)
        } else {
            self.tail_turns.get_mut(index - 1)
        }
    }

    /// Attempts to play a move in the last position.
    ///
    /// The variations capacity of the new turn will be [`VariationsCapacity::default`].
    pub fn play(&mut self, r#move: Move) -> Result<(), VariationPlayError> {
        if self.last_position().is_legal(&r#move) {
            self.tail_turns.push(Turn::new(r#move, VariationsCapacity::default()));
            Ok(())
        } else {
            Err(VariationPlayError {
                at_position: self.tail_turns.len(),
                r#move
            })
        }
    }

    /// Attempts to play a turn in the last position.
    ///
    /// You can use this version of [`play`] to control the variations capacity of the turn.
    pub fn play_turn(&mut self, turn: Turn) -> Result<(), VariationPlayError> {
        if self.last_position().is_legal(&turn.played_move) {
            self.tail_turns.push(turn);
            Ok(())
        } else {
            Err(VariationPlayError {
                at_position: self.tail_turns.len(),
                r#move: turn.played_move
            })
        }
    }

    /// Attempts to convert a SAN to a valid move in the last position, and plays it.
    ///
    /// The variations capacity of the new turn will be [`VariationsCapacity::default`].
    pub fn play_san(&mut self, san: &San) -> Result<(), SanError> {
        let last_position = self.last_position();
        let r#move = san.to_move(last_position.deref())?;

        self.tail_turns.push(Turn::new(r#move, VariationsCapacity::default()));

        Ok(())
    }

    /// Attempts to play a move at the specified index.
    /// This is like an `insert` function, but it will remove all the turns after `index` because
    /// of a change in the variation history.
    pub fn play_at(&mut self, index: usize, r#move: Move) -> Result<(), PlayAtError> {
        if self.get_position(index).ok_or(PlayAtError::NoSuchTurn { index })?.is_legal(&r#move) {
            // CLIPPY: `get_position` verifies that a turn exists at that index too.
            #[allow(clippy::unwrap_used)]
            {
                self.get_turn_mut(index)
                    .unwrap()
                    .played_move = r#move;
            }
            
            for i in index - 1..self.tail_turns.len() {
                self.tail_turns.swap_remove(i);
            }
            
            Ok(())
        } else {
            Err(PlayAtError::PlayError(VariationPlayError {
                at_position: index,
                r#move,
            }))
        }
    }

    /// Calls [`Vec::pop`] on [`Self::tail_turns`].
    pub fn pop(&mut self) -> Option<Turn> {
        self.tail_turns.pop()
    }

    /// Adds a variation to the last turn.
    /// 
    /// The new variation must have the same starting position as this variation's last position.
    pub fn push_variation(&mut self, variation: Self) -> Result<(), PositionDoesNotMatchError> {
        if variation.first_position == *self.last_position() {
            self.last_turn_mut().variations.push(variation);

            Ok(())
        } else {
            Err(PositionDoesNotMatchError)
        }
    }

    /// Inserts a variation to the turn at the specified index.
    /// 
    /// The new variation must have the same starting position as this variation's position at `index`.
    pub fn insert_variation(&mut self, index: usize, variation: Self) -> Result<(), InsertVariationError> {
        if variation.first_position == *self.get_position(index).ok_or(InsertVariationError::NoSuchTurn { index })? {
            self.get_turn_mut(index)
                .ok_or(InsertVariationError::NoSuchTurn { index })?
                .variations
                .push(variation);

            Ok(())
        } else {
            Err(InsertVariationError::PositionDoesNotMatch)
        }
    }

    pub fn insert_empty_variation(&mut self, index: usize) -> Result<&mut Variation, NoSuchTurnError> {
        let position_at_index = self.get_position(index).ok_or(NoSuchTurnError { index })?;

        // CLIPPY: `get_position` verifies that a turn exists at that index too.
        #[allow(clippy::unwrap_used)]
        {
            self.get_turn_mut(index)
                .unwrap()
                .variations
                .push(Variation::from_move(FirstPosition(position_at_index), ).unwrap())
        }
    }
}

/// Plays the given moves in the variation, returning the first error.
///
/// Syntax: `play_moves!(variation, move1, move2, ..)`.
#[macro_export]
macro_rules! play_moves {
    ($variation:expr, $($r#move:expr),*) => {
        {
            use ::shakmaty::{PlayError, Chess};
            use $crate::Variation;
            fn play_moves(variation: &mut Variation) -> Result<(), PlayError<Chess>> {
                $(
                variation.play($r#move)?;
                )*
                
                Ok(())
            }
            
            play_moves(&mut $variation)
        }
    };
}

/// Plays the given SANs in the variation, returning the first error.
///
/// Syntax: `play_sans!(variation, san1, san2, ..)`.
#[macro_export]
macro_rules! play_sans {
    ($variation:expr, $($san:expr),*) => {
        {
            use ::shakmaty::san::SanError;
            use $crate::Variation;
            fn play_sans(variation: &mut Variation) -> Result<(), SanError> {
                $(
                variation.play_san($san)?;
                )*

                Ok(())
            }

            play_sans(&mut $variation)
        }
    };
}

/// Plays the given SAN strings in the variation, returning the first error.
///
/// Syntax: `play_san_strings!(variation, san_string1, san_string2, ..)`.
///
/// # Panics
///
/// Panics if the strings are not correctly formatted SANs.
/// Will not panic if the SAN isn't legal as long as it is correctly formatted.
macro_rules! play_san_strings {
    ($variation:expr, $($san_string:expr),*) => {
        {
            use ::shakmaty::san::SanError;
            use ::std::str::FromStr;
            use $crate::Variation;
            fn play_sans(variation: &mut Variation) -> Result<(), SanError> {
                $(
                variation.play_san(&San::from_str($san_string).unwrap())?;
                )*

                Ok(())
            }

            play_sans(&mut $variation)
        }
    };
}

// This is used in tests.
#[allow(unused_imports)]
pub(crate) use play_moves;
// This is used in tests.
#[allow(unused_imports)]
pub(crate) use play_sans;
// This is used in tests.
#[allow(unused_imports)]
pub(crate) use play_san_strings;
