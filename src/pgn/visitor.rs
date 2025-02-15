use std::borrow::Cow;
use pgn_reader::{RawHeader, Skip};
use std::str::{FromStr, Utf8Error};
use shakmaty::{CastlingMode, Chess, Position, PositionError};
use shakmaty::fen::{Fen, ParseFenError};
use shakmaty::san::SanPlus;
use crate::{LegalVariation, Eco, pgn::{Date, Round, Outcome, Pgn}, TurnsCapacity, VariationsCapacity, VariationSanPlayError};

#[derive(Clone, Debug)]
pub enum RootVariationError {
    InvalidFenPosition(PositionError<Chess>),
    SanPlayError(VariationSanPlayError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Has functions of the [`pgn_reader::RawHeader`], but has ownership of the bytes.
pub struct RawOwnedHeader(pub Vec<u8>);

impl RawOwnedHeader {
    pub fn new(raw_header: RawHeader<'_>) -> Self {
        Self(raw_header.0.to_vec())
    }

    pub fn decode(&self) -> Cow<[u8]> {
        RawHeader(&self.0).decode()
    }

    pub fn decode_utf8(&self) -> Result<Cow<str>, Utf8Error> {
        RawHeader(&self.0).decode_utf8()
    }

    pub fn decode_utf8_lossy(&self) -> Cow<str> {
        RawHeader(&self.0).decode_utf8_lossy()
    }
}

pub(super) struct Visitor {
    event: Option<RawOwnedHeader>,
    site: Option<RawOwnedHeader>,
    date: Option<Result<Date, <Date as FromStr>::Err>>,
    round: Option<Result<Round, <Round as FromStr>::Err>>,
    white: Option<RawOwnedHeader>,
    white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    black: Option<RawOwnedHeader>,
    black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    outcome: Option<Result<Outcome, <Outcome as FromStr>::Err>>,
    eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    time_control: Option<RawOwnedHeader>,
    fen: Option<Result<Fen, ParseFenError>>,
    variation_tree: Vec<(usize, LegalVariation)>,
    current_turn_index: usize,
    movetext_string: String,
    root_variation: Option<Result<LegalVariation, RootVariationError>>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            white_elo: None,
            black: None,
            black_elo: None,
            outcome: None,
            eco: None,
            time_control: None,
            fen: None,
            variation_tree: Vec::with_capacity(0),
            current_turn_index: 0,
            movetext_string: String::new(),
            root_variation: None,
        }
    }

    /// Moves relevant contents of the visitor into a new [`Pgn`].
    ///
    /// Call this after you visit [`Visitor`] with a reader.
    ///
    /// This is done because `pgn_reader`'s `Visitor` trait has a required `end_game`
    /// function, which would ideally return [`Pgn`], but it does not consume the visitor,
    /// so nothing can be moved.
    pub fn into_pgn(self) -> Pgn {
        Pgn {
            event: self.event,
            site: self.site,
            date: self.date,
            white: self.white,
            white_elo: self.white_elo,
            black: self.black,
            black_elo: self.black_elo,
            outcome: self.outcome,
            round: self.round,
            eco: self.eco,
            time_control: self.time_control,
            fen: self.fen,
            root_variation: match self.root_variation {
                None => None,
                Some(Ok(variation)) => Some(Ok(variation)),
                Some(Err(e)) => Some(Err((e, self.movetext_string)))
            }
        }
    }
}

impl pgn_reader::Visitor for Visitor {
    type Result = ();

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        match key.to_ascii_lowercase().as_slice() {
            b"event" => self.event = Some(RawOwnedHeader::new(value)),
            b"site" => self.site = Some(RawOwnedHeader::new(value)),
            b"date" => self.date = Some(Date::from_str(&value.decode_utf8_lossy())),
            b"white" => self.white = Some(RawOwnedHeader::new(value)),
            b"black" => self.black = Some(RawOwnedHeader::new(value)),
            b"whiteelo" => self.white_elo = Some(value.decode_utf8_lossy().parse()),
            b"blackelo" => self.black_elo = Some(value.decode_utf8_lossy().parse()),
            b"result" => self.outcome = Some(Outcome::from_str(&value.decode_utf8_lossy())),
            b"round" => self.round = Some(Round::from_str(&value.decode_utf8_lossy())),
            b"eco" => self.eco = Some(Eco::from_str(&value.decode_utf8_lossy())),
            b"timecontrol" => self.time_control = Some(RawOwnedHeader::new(value)),
            b"fen" => self.fen = Some(Fen::from_str(&value.decode_utf8_lossy())),
            _ => {},
        }
    }

    fn begin_game(&mut self) {
        if let Some(Ok(fen)) = &self.fen {
            match fen.clone().into_position(CastlingMode::detect(&fen.as_setup().clone())) {
                Ok(position) => self.root_variation = Some(Ok(LegalVariation::new(position, TurnsCapacity::default()))),
                Err(e) => self.root_variation = Some(Err(RootVariationError::InvalidFenPosition(e))),
            }
        } else {
            self.root_variation = Some(Ok(LegalVariation::new(Chess::new(), TurnsCapacity::default())));
        }
    }

    fn begin_variation(&mut self) -> Skip {
        self.movetext_string.push('(');
        
        let Some(Ok(root_variation)) = &self.root_variation else {
            return Skip(true);
        };

        self.current_turn_index = self.current_turn_index.saturating_sub(1);

        let current_variation = self.variation_tree.last().map_or(root_variation, |(_, last_variation)| last_variation);
        let new_variation = LegalVariation::new(current_variation.position_before_last_move().clone(), TurnsCapacity(50));

        self.variation_tree.push((self.current_turn_index, new_variation));

        Skip(false)
    }

    fn end_variation(&mut self) {
        self.movetext_string.push(')');
        
        let Some(Ok(root_variation)) = &mut self.root_variation else {
            return;
        };

        // Remove the current variation because it ended, but get the value of it to push to the parent.
        let Some((ending_variation_move_number, ending_variation)) = self.variation_tree.pop() else {
            return;
        };

        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_turn_index = ending_variation_move_number + 1;
        }

        let ending_variation_parent = if let Some((_, last_variation)) = self.variation_tree.last_mut() {
            last_variation
        } else {
            root_variation
        };
        
        // print!("Finishing var: ");
        // 
        // for turn_i in 0..current_variation.turns().len() {
        //     #[allow(clippy::unwrap_used)]
        //     let r#move = current_variation.turns().get(turn_i).unwrap().r#move();
        //     #[allow(clippy::unwrap_used)]
        //     let position = current_variation.get_position(turn_i).unwrap();
        //     
        //     print!("{}, ", San::from_move(&*position, r#move));
        // }
        // 
        // println!();
        
        // CLIPPY: All error cases are covered. `len - 1` will always be a valid index and the position is correct as assured in `begin_variation`.
        #[allow(clippy::unwrap_used)]
        ending_variation_parent.insert_variation(ending_variation_parent.turns().len().saturating_sub(1), ending_variation).unwrap();
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.movetext_string.push_str(&san_plus.to_string());
        
        let Some(Ok(root_variation)) = &mut self.root_variation else {
            return;
        };

        let current_variation = self.variation_tree.last_mut().map(|pair| &mut pair.1).unwrap_or(root_variation);

        //println!("Current variation position: \n{:?}", current_variation.last_position().board());

        if let Err(error) = current_variation.play_san(&san_plus.san, VariationsCapacity::default()) {
            //println!("Move {} is err", san_plus.san);
            self.root_variation = Some(Err(RootVariationError::SanPlayError(VariationSanPlayError {
                turn_index: self.current_turn_index,
                //position: current_variation.last_position().into_owned(),
                san: san_plus.san,
                error: error.error,
            })));
        } else {
            // CLIPPY: There's never going to be usize::MAX moves.
            #[allow(clippy::arithmetic_side_effects)]
            {
                self.current_turn_index += 1;
            }
            //println!("Move {} is ok", san_plus.san)
        }
    }

    fn end_game(&mut self) -> Self::Result {}
}