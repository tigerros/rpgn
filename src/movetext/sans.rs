use shakmaty::san::{San, SanPlus};
use std::fmt::{Display, Formatter, Write};
use crate::{MoveNumber, Movetext};
use crate::movetext::{SanWithVariations, Variation};

/// A vec of SANs. Use if you don't care about variations.
/// 
/// Why is there a generic? To allow usage of either a [`San`] or a [`SanPlus`].
/// A [`San`] is often sufficient and is smaller than a [`SanPlus`], so use a [`San`]
/// unless you need the [`SanPlus`] suffix.
/// 
/// Only implements [`Movetext`] if `S` is [`San`] or [`SanPlus`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sans<S>(pub Vec<S>);

impl<S> Default for Sans<S> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<S> From<Variation<S>> for Sans<S> {
    /// Takes the root variation of the given [`Variation`] and transfers it to a [`Sans`].
    fn from(variation: Variation<S>) -> Self {
        let mut san_vec = Self(Vec::with_capacity(variation.0.len()));

        for SanWithVariations { san, .. } in variation.0 {
            san_vec.0.push(san);
        }

        san_vec
    }
}

impl<S> Display for Sans<S> where S: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut move_number = MoveNumber(0);
        let mut first_move = true;

        for san in &self.0 {
            if first_move {
                first_move = false;
            } else {
                f.write_char(' ')?;
            }

            move_number.number().fmt(f)?;

            if move_number.color().is_white() {
                f.write_str(". ")?;
            } else {
                f.write_str("... ")?;
            }

            san.fmt(f)?;

            // CLIPPY: There will never be u16::MAX moves.
            #[allow(clippy::arithmetic_side_effects)]
            {
                move_number.0 += 1;
            }
        }

        Ok(())
    }
}

macro_rules! base_movetext_impl {
    () => {
        type Agent = Self;

        fn begin_game() -> Self { Self(Vec::with_capacity(100)) }
        fn end_game(agent: Self) -> Self { agent }
    };
}

impl Movetext for Sans<San> {
    base_movetext_impl! {}

    fn san(agent: &mut Self, san: SanPlus) { agent.0.push(san.san); }
}

impl Movetext for Sans<SanPlus> {
    base_movetext_impl! {}
    
    fn san(agent: &mut Self, san: SanPlus) { agent.0.push(san); }
}