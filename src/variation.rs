use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::Deref;
use shakmaty::{Chess, Move, Position};
use shakmaty::san::{San, SanError, SanPlus, Suffix};

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
    r#move: Move,
    variations: Vec<Variation>
}

impl Turn {
    pub fn new(r#move: Move, variations_capacity: VariationsCapacity) -> Self {
        Self {
            r#move,
            variations: Vec::with_capacity(variations_capacity.0)
        }
    }

    pub fn r#move(&self) -> &Move {
        &self.r#move
    }

    pub fn variations(&self) -> &Vec<Variation> {
        &self.variations
    }

    pub fn get_variation_mut(&mut self, index: usize) -> Option<&mut Variation> {
        self.variations.get_mut(index)
    }
}

/// An always legal variation with a history of [`Turn`]s.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variation {
    first_position: Chess,
    turns: Vec<Turn>
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

#[derive(Debug)]
pub enum PlaySanAtError {
    NoSuchTurn { index: usize },
    PlayError(VariationSanPlayError)
}

/// The position does not match the new variation's starting position.
#[derive(Debug, Clone)]
pub struct PositionDoesNotMatchError {
    pub position1: Chess,
    pub position2: Chess
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

#[derive(Debug)]
pub struct VariationSanPlayError {
    pub at_position: usize,
    pub san: San,
    pub error: SanError
}

#[derive(Debug, Clone)]
pub struct TurnsCapacity(pub usize);

impl Default for TurnsCapacity {
    /// 100
    fn default() -> Self {
        Self(100)
    }
}

impl Variation {
    pub fn new(first_position: Chess, turns_capacity: TurnsCapacity) -> Self {
        Self {
            first_position,
            turns: Vec::with_capacity(turns_capacity.0),
        }
    }

    pub fn first_position(&self) -> &Chess {
        &self.first_position
    }

    pub fn turns(&self) -> &Vec<Turn> {
        &self.turns
    }

    /// Returns the position that occurs *after* the last move is played.
    ///
    /// See also [`Self::position_before_last_move`].
    pub fn last_position(&self) -> Cow<Chess> {
        if self.turns.is_empty() {
            return Cow::Borrowed(&self.first_position);
        }

        let mut last_position = self.first_position.clone();

        for turn in &self.turns {
            last_position.play_unchecked(&turn.r#move);
        }

        Cow::Owned(last_position)
    }

    /// Returns the position that occurs *before* the last move is played.
    ///
    /// This is useful if you want to start a subvariation at the last turn of a variation.
    ///
    /// See also [`Self::last_position`].
    pub fn position_before_last_move(&self) -> Cow<Chess> {
        if self.turns.is_empty() {
            return Cow::Borrowed(&self.first_position);
        }

        let mut last_position = self.first_position.clone();

        for turn_i in 0..self.turns.len() - 1 {
            // CLIPPY: This for loop ensures the index is within bounds.
            #[allow(clippy::unwrap_used)]
            let turn = self.turns.get(turn_i).unwrap();
            
            last_position.play_unchecked(&turn.r#move);
        }

        Cow::Owned(last_position)
    }
    
    pub fn get_position(&self, index: usize) -> Option<Cow<Chess>> {
        if index == 0 {
            return Some(Cow::Borrowed(&self.first_position));
        } else if index > self.turns.len() {
            return None;
        }

        let mut requested_position = self.first_position.clone();
        let mut turns_traversed = 0;

        for turn in &self.turns {
            if index == turns_traversed {
                break;
            }

            requested_position.play_unchecked(&turn.r#move);

            turns_traversed += 1;
        }

        Some(Cow::Owned(requested_position))
    }

    pub fn get_turn_mut(&mut self, index: usize) -> Option<&mut Turn> {
        self.turns.get_mut(index)
    }

    /// Attempts to play a turn in the last position.
    pub fn play(&mut self, turn: Turn) -> Result<(), VariationPlayError> {
        if !self.last_position().is_legal(&turn.r#move) {
            return Err(VariationPlayError {
                at_position: self.turns.len(),
                r#move: turn.r#move
            });
        }

        self.turns.push(turn);
        Ok(())
    }

    /// See [`Self::play`].
    pub fn play_san(&mut self, san: &San, variations_capacity: VariationsCapacity) -> Result<(), SanError> {
        let last_position = self.last_position();
        let r#move = san.to_move(last_position.deref())?;

        self.turns.push(Turn::new(r#move, variations_capacity));

        Ok(())
    }

    /// Attempts to play a move at the specified index,
    /// thereby changing the move at that index and removing all the turns after `index`.
    ///
    /// Removal is necessary because the variation history is changed by this.
    pub fn play_at(&mut self, index: usize, r#move: Move) -> Result<(), PlayAtError> {
        if !self.get_position(index).ok_or(PlayAtError::NoSuchTurn { index })?.is_legal(&r#move) {
            return Err(PlayAtError::PlayError(VariationPlayError {
                at_position: index,
                r#move,
            }));
        }

        // CLIPPY: `get_position` verifies that a turn exists at that index too.
        #[allow(clippy::unwrap_used)]
        {
            self.get_turn_mut(index)
                .unwrap()
                .r#move = r#move;
        }

        for i in index + 1..self.turns.len() {
            self.turns.swap_remove(i);
        }

        Ok(())
    }

    /// See [`Self::play_at`].
    pub fn play_san_at(&mut self, index: usize, san: San) -> Result<(), PlaySanAtError> {
        let r#move = san.to_move(self.get_position(index).ok_or(PlaySanAtError::NoSuchTurn { index })?.deref()).map_err(|error| PlaySanAtError::PlayError(VariationSanPlayError {
            at_position: index,
            san,
            error,
        }))?;

        // CLIPPY: `get_position` verifies that a turn exists at that index too.
        #[allow(clippy::unwrap_used)]
        {
            self.get_turn_mut(index)
                .unwrap()
                .r#move = r#move;
        }

        for i in index + 1..self.turns.len() {
            self.turns.swap_remove(i);
        }

        Ok(())
    }

    /// See [`Vec::pop`].
    pub fn pop(&mut self) -> Option<Turn> {
        self.turns.pop()
    }

    /// Inserts a variation to the turn at the specified index.
    /// 
    /// The new variation must have the same starting position as this variation's position at `index`.
    pub fn insert_variation(&mut self, index: usize, variation: Self) -> Result<(), InsertVariationError> {
        if variation.first_position != *self.get_position(index).ok_or(InsertVariationError::NoSuchTurn { index })? {
            return Err(InsertVariationError::PositionDoesNotMatch);
        }

        self.get_turn_mut(index)
            .ok_or(InsertVariationError::NoSuchTurn { index })?
            .variations
            .push(variation);

        Ok(())
    }
}

fn fmt(f: &mut Formatter<'_>, mut move_number: MoveNumber, variation: &Variation, mut very_first_move: bool) -> std::fmt::Result {
    for turn_i in 0..variation.turns.len() {
        // CLIPPY: The above for loop ensures the index is within bounds.
        #[allow(clippy::unwrap_used)]
        let Turn { r#move, variations: subvariations } = variation.turns.get(turn_i).unwrap();
        #[allow(clippy::unwrap_used)]
        let position = variation.get_position(turn_i).unwrap();

        if very_first_move {
            very_first_move = false;
        } else {
            f.write_char(' ')?;
        }

        f.write_str(&move_number.number().to_string())?;

        if move_number.color().is_white() {
            f.write_str(". ")?;
        } else {
            f.write_str("... ")?;
        }

        f.write_str(&SanPlus {
            san: San::from_move(&*position, r#move),
            suffix: Suffix::from_position(&*position),
        }.to_string())?;

        for subvariation in subvariations {
            f.write_str(" (")?;
            fmt(f, move_number, subvariation, false)?;
            f.write_str(" )")?;
        }

        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            move_number.index += 1;
        }
    }

    Ok(())
}

impl Display for Variation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt(f, MoveNumber::MIN, self, true)
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
                variation.play_san(&San::from_str($san_string).unwrap(), VariationsCapacity::default())?;
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
use crate::MoveNumber;
