use crate::MoveNumber;
use shakmaty::san::SanPlus;
use std::fmt::{Display, Formatter, Write};
use pgn_reader::Skip;

/// The trait for making a movetext using the structure of the [`pgn_reader::Visitor`].
/// The [`Pgn`] struct requires a generic parameter which implements this trait.
pub trait Movetext {
    type Output;
    fn begin_game() -> Self where Self: Sized;
    fn begin_variation(&mut self) -> Skip;
    fn end_variation(&mut self) {}
    fn san(&mut self, san: SanPlus);
    fn end_game(self) -> Self::Output;
}

/// Use if you don't care about variations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleMovetext(pub Vec<SanPlus>);

impl Display for SimpleMovetext {
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

impl Movetext for SimpleMovetext {
    type Output = Self;
    
    fn begin_game() -> Self {
        Self(Vec::with_capacity(100))
    }

    fn begin_variation(&mut self) -> pgn_reader::Skip {
        Skip(true)
    }

    fn san(&mut self, san: SanPlus) {
        self.0.push(san);
    }

    fn end_game(self) -> Self::Output {
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Use if you do care about variations.
/// Each item in the vec is a move and then the variations occurring on that move.
///
/// *Note: this struct does not implement [`Movetext`]. Use [`VariationMovetextImpl`] for that.*
pub struct VariationMovetext(pub Vec<(SanPlus, Vec<Self>)>);

impl From<SimpleMovetext> for VariationMovetext {
    fn from(simple_movetext: SimpleMovetext) -> Self {
        let mut variation_movetext = Self(Vec::with_capacity(simple_movetext.0.len()));

        for san in simple_movetext.0 {
            variation_movetext.0.push((san, Vec::new()));
        }

        variation_movetext
    }
}

#[derive(Debug)]
/// See [`VariationMovetext`].
pub struct VariationMovetextImpl {
    root_variation: VariationMovetext,
    variation_layers: Vec<VariationMovetext>
}

impl Movetext for VariationMovetextImpl {
    type Output = VariationMovetext;

    fn begin_game() -> Self {
        Self {
            root_variation: VariationMovetext(Vec::new()),
            variation_layers: Vec::new()
        }
    }

    fn begin_variation(&mut self) -> Skip {
        self.variation_layers.push(VariationMovetext(Vec::new()));

        Skip(false)
    }

    fn end_variation(&mut self) {
        let Some(ending_variation) = self.variation_layers.pop() else {
            return;
        };

        let ending_variation_parent = self.variation_layers.last_mut().unwrap_or(&mut self.root_variation);

        #[allow(clippy::unwrap_used)]
        ending_variation_parent.0.last_mut().unwrap().1.push(ending_variation);
    }

    fn san(&mut self, san: SanPlus) {
        let current_variation = self.variation_layers.last_mut().unwrap_or(&mut self.root_variation);

        current_variation.0.push((san, Vec::new()));
    }

    fn end_game(self) -> Self::Output {
        self.root_variation
    }
}

impl VariationMovetext {
    fn fmt(&self, f: &mut Formatter<'_>, mut move_number: MoveNumber, mut very_first_move: bool) -> std::fmt::Result {
        for (san, variations) in &self.0 {
            if very_first_move {
                very_first_move = false;
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

            for variation in variations {
                f.write_str(" (")?;

                variation.fmt(f, move_number, false)?;
                f.write_str(" )")?;
            }

            // CLIPPY: There's never going to be u16::MAX moves.
            #[allow(clippy::arithmetic_side_effects)]
            {
                move_number.0 += 1;
            }
        }

        Ok(())
    }
}

impl Display for VariationMovetext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, MoveNumber::MIN, true)
    }
}