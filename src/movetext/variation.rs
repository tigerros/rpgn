use std::fmt::{Display, Formatter, Write};
use pgn_reader::Skip;
use shakmaty::san::{San, SanPlus};
use crate::{MoveNumber, Movetext, movetext::Sans};

/// See [`Variation`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SanWithVariations<S> {
    pub san: S,
    pub variations: Vec<Variation<S>>,
}

/// A vec of SANs with variations. Use if you do care about variations.
///
/// Regarding the generic `S`, see the docs for [`Sans`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variation<S>(pub Vec<SanWithVariations<S>>);

impl<S> From<Sans<S>> for Variation<S> {
    fn from(san_vec: Sans<S>) -> Self {
        let mut variation_movetext = Self(Vec::with_capacity(san_vec.0.len()));

        for san in san_vec.0 {
            variation_movetext.0.push(SanWithVariations { san, variations: Vec::new() });
        }

        variation_movetext
    }
}

impl<S> Variation<S> where S: Display {
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

impl<S> Display for Variation<S> where S: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, MoveNumber::MIN, true)
    }
}

#[derive(Debug)]
#[doc(hidden)]
/// Only used in [`Variation`]'s implementation of [`Movetext`].
pub struct VariationAgent<S> {
    root_variation: Variation<S>,
    variation_layers: Vec<Variation<S>>
}

macro_rules! base_movetext_impl {
    () => {
        fn begin_game() -> Self::Agent {
            VariationAgent {
                root_variation: Self(Vec::new()),
                variation_layers: Vec::new()
            }
        }

        fn begin_variation(agent: &mut Self::Agent) -> Skip {
            agent.variation_layers.push(Self(Vec::new()));

            Skip(false)
        }

        fn end_variation(agent: &mut Self::Agent) {
            let Some(ending_variation) = agent.variation_layers.pop() else {
                return;
            };

            let ending_variation_parent = agent.variation_layers.last_mut().unwrap_or(&mut agent.root_variation);

            #[allow(clippy::unwrap_used)]
            ending_variation_parent.0.last_mut().unwrap().variations.push(ending_variation);
        }

        fn end_game(agent: Self::Agent) -> Self {
            agent.root_variation
        }
    };
}

impl Movetext for Variation<San> {
    type Agent = VariationAgent<San>;

    fn san(agent: &mut Self::Agent, san: SanPlus) {
        let current_variation = agent.variation_layers.last_mut().unwrap_or(&mut agent.root_variation);

        current_variation.0.push(SanWithVariations { san: san.san, variations: Vec::new() });
    }

    base_movetext_impl! {}
}

impl Movetext for Variation<SanPlus> {
    type Agent = VariationAgent<SanPlus>;

    fn san(agent: &mut Self::Agent, san: SanPlus) {
        let current_variation = agent.variation_layers.last_mut().unwrap_or(&mut agent.root_variation);

        current_variation.0.push(SanWithVariations { san, variations: Vec::new() });
    }

    base_movetext_impl! {}
}