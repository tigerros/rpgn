use crate::MoveNumber;
use shakmaty::san::SanPlus;
use std::fmt::{Display, Formatter, Write};
use pgn_reader::Skip;

/// The trait for making a movetext using the structure of the [`pgn_reader::Visitor`].
///
/// The [`crate::Pgn::from_reader`] function requires a generic which implements this trait.
/// The `Pgn.movetext` field is the [`Movetext::Output`].
///
/// See [`SanVec`] and [`Variation`].
pub trait Movetext {
    type Output;

    fn begin_game() -> Self;
    fn begin_variation(&mut self) -> Skip;
    fn end_variation(&mut self) {}
    fn san(&mut self, san: SanPlus);
    // fn foo(self) -> Self { self } is actually a noop on -Copt-level=3.
    // I think it should apply to traits too, since they're just sugar.
    fn end_game(self) -> Self::Output;
}

/// Use if you don't care about variations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SanVec(pub Vec<SanPlus>);

/// Create a [`SanVec`] out of a list SAN literals.
/// 
/// # Panics
/// See [`SanPlus::from_ascii`].
#[macro_export]
macro_rules! san_vec {
    ($($san:literal),+) => {
        SanVec(vec![$(SanPlus::from_ascii($san).unwrap()),+])
    };
}

impl Display for SanVec {
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

impl Movetext for SanVec {
    type Output = Self;

    fn begin_game() -> Self { Self(Vec::with_capacity(100)) }
    fn begin_variation(&mut self) -> Skip { Skip(true) }
    fn san(&mut self, san: SanPlus) { self.0.push(san); }
    fn end_game(self) -> Self::Output { self }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SanWithVariations {
    pub san: SanPlus,
    pub variations: Vec<Variation>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Use if you do care about variations.
///
/// *Note: this struct does not implement [`Movetext`]. Use [`VariationMovetext`] for that,
/// which has this struct as the [`Movetext::Output`].*
pub struct Variation(pub Vec<SanWithVariations>);

/// Create a [`Variation`] out of SAN literals.
/// 
/// # Syntax
/// See the `samples.rs` file in the repository.
/// 
/// # Panics
/// See [`SanPlus::from_ascii`].
#[macro_export]
macro_rules! variation {
    (_turn: $san:literal) => {
        SanWithVariations { san: SanPlus::from_ascii($san).unwrap(), variations: vec![] }
    };
    (_turn: ($san:literal, [$($vars:tt),+])) => {
        SanWithVariations { san: SanPlus::from_ascii($san).unwrap(), variations: vec![$(variation! $vars),+] }
    };
    {$($turn:tt),+} => {
        Variation(vec![$(variation!(_turn: $turn)),+])
    };
}

impl From<SanVec> for Variation {
    fn from(simple_movetext: SanVec) -> Self {
        let mut variation_movetext = Self(Vec::with_capacity(simple_movetext.0.len()));

        for san in simple_movetext.0 {
            variation_movetext.0.push(SanWithVariations { san, variations: Vec::new() });
        }

        variation_movetext
    }
}

#[derive(Debug)]
/// See [`Variation`].
pub struct VariationMovetext {
    root_variation: Variation,
    variation_layers: Vec<Variation>
}

impl Movetext for VariationMovetext {
    type Output = Variation;

    fn begin_game() -> Self {
        Self {
            root_variation: Variation(Vec::new()),
            variation_layers: Vec::new()
        }
    }

    fn begin_variation(&mut self) -> Skip {
        self.variation_layers.push(Variation(Vec::new()));

        Skip(false)
    }

    fn end_variation(&mut self) {
        let Some(ending_variation) = self.variation_layers.pop() else {
            return;
        };

        let ending_variation_parent = self.variation_layers.last_mut().unwrap_or(&mut self.root_variation);

        #[allow(clippy::unwrap_used)]
        ending_variation_parent.0.last_mut().unwrap().variations.push(ending_variation);
    }

    fn san(&mut self, san: SanPlus) {
        let current_variation = self.variation_layers.last_mut().unwrap_or(&mut self.root_variation);

        current_variation.0.push(SanWithVariations { san, variations: Vec::new() });
    }

    fn end_game(self) -> Self::Output {
        self.root_variation
    }
}

impl Variation {
    fn fmt(&self, f: &mut Formatter<'_>, mut move_number: MoveNumber, mut very_first_move: bool) -> std::fmt::Result {
        for SanWithVariations { san, variations } in &self.0 {
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

            // CLIPPY: There's never going to be usize::MAX moves.
            #[allow(clippy::arithmetic_side_effects)]
            {
                move_number.0 += 1;
            }
        }

        Ok(())
    }
}

impl Display for Variation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, MoveNumber::MIN, true)
    }
}

pub(crate) use san_vec;
pub(crate) use variation;