use std::fmt::{Display, Formatter, Write};
use pgn_reader::Skip;
use shakmaty::san::SanPlus;
use crate::{MoveNumber, Movetext, movetext::Sans};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SanWithVariations {
    pub san: SanPlus,
    pub variations: Vec<Variation>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Use if you do care about variations.
pub struct Variation(pub Vec<SanWithVariations>);

impl From<Sans> for Variation {
    fn from(san_vec: Sans) -> Self {
        let mut variation_movetext = Self(Vec::with_capacity(san_vec.0.len()));

        for san in san_vec.0 {
            variation_movetext.0.push(SanWithVariations { san, variations: Vec::new() });
        }

        variation_movetext
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

#[derive(Debug)]
#[doc(hidden)]
/// Only used in [`Variation`]'s implementation of [`Movetext`].
pub struct VariationAgent {
    root_variation: Variation,
    variation_layers: Vec<Variation>
}

impl Movetext for Variation {
    type Agent = VariationAgent;

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

    fn san(agent: &mut Self::Agent, san: SanPlus) {
        let current_variation = agent.variation_layers.last_mut().unwrap_or(&mut agent.root_variation);

        current_variation.0.push(SanWithVariations { san, variations: Vec::new() });
    }

    fn end_game(agent: Self::Agent) -> Self {
        agent.root_variation
    }
}