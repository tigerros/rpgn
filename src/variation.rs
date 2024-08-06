use std::borrow::Cow;
use std::collections::BTreeMap;
use std::env::var;
use std::fmt::Debug;
use std::iter::once;
use shakmaty::{Chess, Move, PlayError, Position};
use shakmaty::san::{San, SanError};
use crate::MoveNumber;
use crate::Turn;

/// An always legal variation with a history of [`Turn`]s.
///
/// That's why there is no `insert(i, turn)` function, because that would invalidate the turns after `i`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variation {
    first_turn: Turn,
    tail_turns: Vec<Turn>,
    first_position: Chess,
}

#[derive(Debug)]
pub enum PushVariationError {
    NoSuchTurn { index: usize },
    PlayError(PlayError<Chess>)
}

pub struct VariationPlayError<'variation> {
    pub position: &'variation Chess,
    pub r#move: Move,
}

impl Variation {
    pub fn new(first_turn: Turn, tail_turns_capacity: usize) -> Self {
        Self {
            first_turn: first_turn.clone(),
            tail_turns: Vec::with_capacity(tail_turns_capacity),
            first_position: first_turn.position().clone()
        }
    }

    /// Uses 100 for the tail turns capacity.
    pub fn with_default_capacity(first_turn: Turn) -> Self {
        Self {
            first_turn: first_turn.clone(),
            tail_turns: Vec::with_capacity(100),
            first_position: first_turn.position().clone()
        }
    }

    /// The first turn + the tail turns.
    pub fn turns(&self) -> impl Iterator<Item = &Turn> {
        once(&self.first_turn).chain(self.tail_turns.iter())
    }

    /// This will treat the first turn and the tail turns as one list.
    /// Therefore, an index of 0 will yield the first turn, and a greater index will yield `tail_turnns.get(index - 1)`.
    pub fn get_turn(&self, index: usize) -> Option<&Turn> {
        if index == 0 {
            Some(&self.first_turn)
        } else {
            self.tail_turns.get(index - 1)
        }
    }

    /// See [`get_turn`].
    pub fn get_turn_mut(&mut self, index: usize) -> Option<&mut Turn> {
        if index == 0 {
            Some(&mut self.first_turn)
        } else {
            self.tail_turns.get_mut(index - 1)
        }
    }

    pub const fn first_turn(&self) -> &Turn {
        &self.first_turn
    }

    pub fn first_turn_mut(&mut self) -> &mut Turn {
        &mut self.first_turn
    }

    /// Gets the last turn in the tail or the first turn if the tail is empty.
    pub fn last_turn(&self) -> &Turn {
        &self.tail_turns.last().unwrap_or(&self.first_turn)
    }

    /// See [`last_turn`].
    pub fn last_turn_mut(&mut self) -> &mut Turn {
        if let Some(last) = self.tail_turns.last_mut() {
            last
        } else {
            &mut self.first_turn
        }
    }

    /// The turns that follow after the first turn.
    pub const fn tail_turns(&self) -> &Vec<Turn> {
        &self.tail_turns
    }

    /// Attempts to play a move in the last position.
    pub fn play(&mut self, r#move: Move) -> Result<(), VariationPlayError> {
        let last_position = self.last_turn().position();
        
        if !last_position.is_legal(&r#move) {
            return Err(VariationPlayError {
                position: self.last_turn().position(),
                r#move
            });
        }

        let mut new_position = last_position.clone();
        new_position.play_unchecked(&r#move);

        self.tail_turns.push(Turn::with_default_capacity(new_position, r#move));

        Ok(())
    }

    /// Attempts to convert a SAN to a valid move in the last position.
    pub fn play_san(&mut self, san: San) -> Result<(), SanError> {
        let last_position = self.last_turn().position();
        let r#move = san.to_move(last_position)?;
        let mut new_position = last_position.clone();

        // This is safe because `San::to_move` guarantees legality.
        new_position.play_unchecked(&r#move);

        Ok(())
    }

    pub fn last_position(&self) -> Cow<Chess> {
        if self.tail_turns.is_empty() {
            return Cow::Borrowed(&self.first_position);
        }

        let mut last_position = self.first_position.clone();

        for r#move in &self.tail_turns {
            let Turn { played_move: r#move, .. } = r#move;
            
            last_position.play_unchecked(&r#move);
        }
        
        Cow::Owned(last_position)
    }

    pub fn play2(&mut self, r#move: Move) -> Result<(), VariationPlayError> {
        let last_position = self.last_position();
        
        if last_position.is_legal(&r#move) {
            self.tail_turns.push(Turn::with_default_capacity(Chess::new(), r#move));
            Ok(())
        } else {
            Err(VariationPlayError {
                position: self.last_turn().position(),
                r#move
            })
        }
    }

    // /// Attempts to play a move at the specified index.
    // /// This is like an `insert` function, but it will remove all the turns
    // pub fn play_at(&mut self, index: usize, r#move: Move) -> Result<(), PlayError<Chess>> {
    //
    // }

    /// Removes the last turn from the tail.
    pub fn pop(&mut self) -> Option<Turn> {
        self.tail_turns.pop()
    }
    
    pub fn push_variation_at(&mut self, index: usize, variation: Self) -> Result<(), PushVariationError> {
        self
            .get_turn_mut(index)
            .ok_or(PushVariationError::NoSuchTurn { index })?
            .push_variation(variation)
            .map_err(PushVariationError::PlayError)?;

        Ok(())
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
                variation.play_san(San::from_str($san_string).unwrap())?;
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
