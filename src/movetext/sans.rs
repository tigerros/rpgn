use shakmaty::san::SanPlus;
use std::fmt::{Display, Formatter, Write};
use crate::{MoveNumber, Movetext};

/// Use if you don't care about variations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sans(pub Vec<SanPlus>);

use crate::movetext::{SanWithVariations, Variation};

impl From<Variation> for Sans {
    /// Takes the root variation of the given [`Variation`] and transfers it to a [`Sans`].
    fn from(variation: Variation) -> Self {
        let mut san_vec = Self(Vec::with_capacity(variation.0.len()));

        for SanWithVariations { san, .. } in variation.0 {
            san_vec.0.push(san);
        }

        san_vec
    }
}

impl Display for Sans {
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

impl Movetext for Sans {
    type Agent = Self;

    fn begin_game() -> Self { Self(Vec::with_capacity(100)) }
    fn san(agent: &mut Self, san: SanPlus) { agent.0.push(san); }
    fn end_game(agent: Self) -> Self { agent }
}