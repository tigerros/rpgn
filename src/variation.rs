use std::fmt::{Debug, Display, Formatter, Write};
use shakmaty::{Chess, Move, Position};
use shakmaty::san::{San, SanError, SanPlus, Suffix};

#[derive(Debug, Clone, Copy)]
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
    variations: Vec<Variation>,
    position_after: Chess,
}

impl Turn {
    fn new(r#move: Move, variations_capacity: VariationsCapacity, position_after: Chess) -> Self {
        Self {
            r#move,
            variations: Vec::with_capacity(variations_capacity.0),
            position_after,
        }
    }

    pub const fn r#move(&self) -> &Move {
        &self.r#move
    }

    pub const fn variations(&self) -> &Vec<Variation> {
        &self.variations
    }

    pub const fn position_after(&self) -> &Chess {
        &self.position_after
    }

    pub fn get_variation_mut(&mut self, index: usize) -> Option<&mut Variation> {
        self.variations.get_mut(index)
    }
}

/// An always legal variation with a history of [`Turn`]s.
///
/// Position indexes are treated as such: the position at turn index `i` is the position that occurs
/// *before* the move at turn index `i` is played.
/// To get the position *after* the last move was played, you can use [`Variation::position_after_last_move`],
/// or `Variation::get_position(Variation::turns.len())`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variation {
    first_position: Chess,
    turns: Vec<Turn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertVariationError {
    NoSuchTurn { index: usize },
    /// The position at the specified index does not match the new variation's starting position.
    PositionDoesNotMatch {
        position_at_index: Box<Chess>,
        new_variation_first_position: Box<Chess>
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayAtError {
    NoTurnAt { index: usize },
    /// An illegal move was played.
    PlayError(VariationPlayError)
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlaySanAtError {
    NoTurnAt { index: usize },
    /// An illegal/ambiguous SAN was played.
    PlayError(VariationSanPlayError)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NoSuchTurnError {
    pub index: usize
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariationPlayError {
    pub turn_index: usize,
    pub r#move: Move,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariationSanPlayError {
    pub turn_index: usize,
    pub san: San,
    pub error: SanError
}

#[derive(Debug, Clone, Copy)]
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

    pub const fn first_position(&self) -> &Chess {
        &self.first_position
    }

    pub const fn turns(&self) -> &Vec<Turn> {
        &self.turns
    }

    /// Returns the position that occurs *after* the last move is played.
    ///
    /// See also [`Self::position_before_last_move`] and [`Self::get_position`].
    pub fn position_after_last_move(&self) -> &Chess {
        self.turns.last().map_or_else(|| self.first_position(), |last_turn| last_turn.position_after())
    }

    /// Returns the position that occurs *before* the last move is played.
    ///
    /// This is useful if you want to start a subvariation at the last turn of a variation.
    ///
    /// See also [`Self::position_after_last_move`] and [`Self::get_position`].
    pub fn position_before_last_move(&self) -> &Chess {
        self.turns.get(self.turns.len().overflowing_sub(2).0).map_or(&self.first_position, |second_last_turn| second_last_turn.position_after())
    }

    /// Returns the position that occurs before the turn at `index` is played,
    /// or [`None`] if the index is out of bounds.
    ///
    /// See also [`Self::position_before_last_move`] and [`Self::position_after_last_move`].
    pub fn get_position(&self, index: usize) -> Option<&Chess> {
        if index == 0 || self.turns.is_empty() {
            Some(&self.first_position)
        } else {
            // CLIPPY: `index > 0`, so `index - 1 > -1`.
            #[allow(clippy::arithmetic_side_effects)]
            self.turns.get(index - 1).map(Turn::position_after)
        }
    }

    /// Equivalent to [`Vec::get_mut`].
    ///
    /// Useful if you need to modify a variation at that turn.
    pub fn get_turn_mut(&mut self, index: usize) -> Option<&mut Turn> {
        self.turns.get_mut(index)
    }

    /// Attempts to play a move in the last position.
    ///
    /// # Errors
    ///
    /// See [`VariationPlayError`].
    pub fn play(&mut self, r#move: Move, variations_capacity: VariationsCapacity) -> Result<(), VariationPlayError> {
        let position_after_last_move = self.position_after_last_move();

        if !position_after_last_move.is_legal(&r#move) {
            return Err(VariationPlayError {
                turn_index: self.turns.len(),
                r#move
            });
        }

        let mut new_position = position_after_last_move.clone();
        new_position.play_unchecked(&r#move);

        self.turns.push(Turn::new(r#move, variations_capacity, new_position));

        Ok(())
    }

    /// See [`Self::play`].
    ///
    /// # Errors
    ///
    /// See [`VariationSanPlayError`]. `at_position` is set to the last turn index in this variation.
    pub fn play_san(&mut self, san: &San, variations_capacity: VariationsCapacity) -> Result<(), VariationSanPlayError> {
        let position_after_last_move = self.position_after_last_move();
        let r#move = san.to_move(position_after_last_move).map_err(|error| VariationSanPlayError {
            turn_index: self.turns.len().saturating_sub(1),
            san: san.clone(),
            error,
        })?;

        let mut new_position = position_after_last_move.clone();
        new_position.play_unchecked(&r#move);

        self.turns.push(Turn::new(r#move, variations_capacity, new_position));

        Ok(())
    }

    // CLIPPY: All potential panicking code is explained.
    #[allow(clippy::missing_panics_doc)]
    /// Attempts to play a move at the specified index,
    /// thereby changing the move at that index and removing all the turns after `index`.
    ///
    /// Removal is necessary because the variation history is changed by this.
    ///
    /// # Errors
    ///
    /// See [`PlayAtError`].
    pub fn play_at(&mut self, index: usize, r#move: Move) -> Result<(), PlayAtError> {
        let position_at_index = self.get_position(index).ok_or(PlayAtError::NoTurnAt { index })?;
        
        if !position_at_index.is_legal(&r#move) {
            return Err(PlayAtError::PlayError(VariationPlayError {
                turn_index: index,
                r#move,
            }));
        }

        let mut new_position = position_at_index.clone();
        new_position.play_unchecked(&r#move);

        // CLIPPY: `get_position` verifies that a turn exists at that index too.
        #[allow(clippy::unwrap_used)]
        let turn = self.get_turn_mut(index).unwrap();
        turn.r#move = r#move;
        turn.position_after = new_position;
        
        self.turns.drain(index.saturating_add(1)..self.turns.len());

        Ok(())
    }

    // CLIPPY: This function never panics; all panicking functions are explained.
    #[allow(clippy::missing_panics_doc)]
    /// See [`Self::play_at`].
    ///
    /// # Errors
    ///
    /// See [`PlaySanAtError`].
    pub fn play_san_at(&mut self, index: usize, san: &San) -> Result<(), PlaySanAtError> {
        let position_at_index = self.get_position(index).ok_or(PlaySanAtError::NoTurnAt { index })?;
        let r#move = san.to_move(position_at_index).map_err(|error| PlaySanAtError::PlayError(VariationSanPlayError {
            turn_index: index,
            san: san.clone(),
            error,
        }))?;

        let mut new_position = position_at_index.clone();
        new_position.play_unchecked(&r#move);

        // CLIPPY: `get_position` verifies that a turn exists at that index too.
        #[allow(clippy::unwrap_used)]
        let turn = self.get_turn_mut(index).unwrap();
        turn.r#move = r#move;
        turn.position_after = new_position;

        self.turns.drain(index.saturating_add(1)..self.turns.len());

        Ok(())
    }

    /// See [`Vec::pop`].
    pub fn pop(&mut self) -> Option<Turn> {
        self.turns.pop()
    }

    /// Inserts a variation to the turn at the specified index.
    /// 
    /// The new variation must have the same starting position as this variation's position at `index`.
    ///
    /// # Errors
    ///
    /// See [`InsertVariationError`].
    pub fn insert_variation(&mut self, index: usize, variation: Self) -> Result<(), InsertVariationError> {
        let position_at_index = self.get_position(index).ok_or(InsertVariationError::NoSuchTurn { index })?.clone();

        if variation.first_position != position_at_index {
            return Err(InsertVariationError::PositionDoesNotMatch {
                position_at_index: Box::new(position_at_index),
                new_variation_first_position: Box::new(variation.first_position)
            });
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
        let Turn { position_after, r#move, variations: subvariations } = variation.turns.get(turn_i).unwrap();

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

        // You would think that using the position *after* the move would mess up `San::from_move`,
        // but nope. Tests pass.
        f.write_str(&SanPlus {
            san: San::from_move(position_after, r#move),
            suffix: Suffix::from_position(position_after),
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
    /// Displays the PGN movelist representation of this variation.
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
            use $crate::{Variation, VariationsCapacity};
            fn play_sans(variation: &mut Variation) -> Result<(), SanError> {
                $(
                variation.play_san($san, VariationsCapacity::default())?;
                )*

                Ok(())
            }

            play_sans(&mut $variation)
        }
    };
}

// It is used in tests.
#[allow(unused_macros)]
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
            use ::shakmaty::san::San;
            use ::std::str::FromStr;
            use $crate::{Variation, VariationSanPlayError, VariationsCapacity};
            fn play_sans(variation: &mut Variation) -> Result<(), VariationSanPlayError> {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::print_stdout)]
mod tests {
    use super::*;
    use crate::samples::*;
    use test_case::test_case;
    use pretty_assertions::assert_eq;
    
    #[test_case(&variation_sample0())]
    #[test_case(&variation_sample1())]
    #[test_case(&variation_sample2())]
    #[test_case(&variation_sample6())]
    fn position_before_last_move(var: &Variation) {
        let mut position = var.first_position.clone();
        
        for turn_i in 0..var.turns.len().saturating_sub(1) {
            // CLIPPY: `turn_i` is within bounds.
            #[allow(clippy::indexing_slicing)]
            let turn = &var.turns[turn_i];
            
            position.play_unchecked(turn.r#move());
        }
        
        println!("position_before_last_move: {:?}", var.position_before_last_move().board());
        println!("correct position: {:?}", position.board());
        
        assert_eq!(var.position_before_last_move(), &position);
    }

    #[test_case(&variation_sample0())]
    #[test_case(&variation_sample1())]
    #[test_case(&variation_sample2())]
    #[test_case(&variation_sample6())]
    fn position_after_last_move(var: &Variation) {
        let mut position = var.first_position.clone();

        for turn in var.turns() {
            position.play_unchecked(turn.r#move());
        }

        println!("position_after_last_move: {:?}", var.position_after_last_move().board());
        println!("correct position: {:?}", position.board());

        assert_eq!(var.position_after_last_move(), &position);
    }
    
    #[test_case(&variation_sample0())]
    #[test_case(&variation_sample1())]
    #[test_case(&variation_sample2())]
    #[test_case(&variation_sample6())]
    fn get_position(var: &Variation) {
        for i in 0..=var.turns().len() {
            let mut position = Chess::new();

            // CLIPPY: Maximum value of `i` is `turns.len()`.
            // Ranges are exclusive. Therefore this range (inclusive) is `0..turns.len - 1`, which is always valid.
            #[allow(clippy::indexing_slicing)]
            for r#move in var.turns()[0..i].iter().map(Turn::r#move) {
                position.play_unchecked(r#move);
            }
            
            println!("get_position[{i}]: {:?}", var.get_position(i).map(Position::board));
            println!("correct position: {:?}", &position.board());
            assert_eq!(var.get_position(i), Some(&position));
        }
    }

    #[test_case(variation_sample0())]
    #[test_case(variation_sample1())]
    #[test_case(variation_sample2())]
    #[test_case(variation_sample6())]
    fn play_at(mut var: Variation) {
        if var.turns().len() <= 1 {
            if let Some(legal_move) = var.first_position().legal_moves().first() {
                var.play_at(0, legal_move.clone()).unwrap();
            }

            get_position(&var);
            position_before_last_move(&var);
            position_after_last_move(&var);
            return;
        }

        // Unwrapping here never panics because the turns length > 1 so position exists at index 1.
        let legal_moves = var.get_position(1).unwrap().legal_moves();
        let Some(legal_move) = legal_moves.first() else {
            return;
        };

        var.play_at(1, legal_move.clone()).unwrap();

        assert_eq!(var.turns().len(), 2);

        get_position(&var);
        position_before_last_move(&var);
        position_after_last_move(&var);
    }
}