use pgn_reader::{RawHeader, Skip};
use std::str::FromStr;
use shakmaty::san::SanPlus;
use crate::{Variation, MoveNumber, Eco, game::{Date, Round, Outcome, Game}, SanErrorWithMoveNumber};

pub(super) struct Visitor {
    event: Option<String>,
    site: Option<String>,
    date: Option<Date>,
    round: Option<Round>,
    white: Option<String>,
    white_elo: Option<u16>,
    black: Option<String>,
    black_elo: Option<u16>,
    outcome: Option<Outcome>,
    eco: Option<Eco>,
    time_control: Option<String>,
    current_variation_tree: Vec<Variation>,
    current_move_number: MoveNumber,
    root_variation: Variation,
    result: Result<(), SanErrorWithMoveNumber>
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
            current_variation_tree: Vec::with_capacity(0),
            current_move_number: MoveNumber::MIN,
            root_variation: Variation::new_starting_root_variation(),
            result: Ok(())
        }
    }

    /// Moves relevant contents of the visitor into a new `Game`.
    ///
    /// Call this after you visit `GameVisitor` with a reader.
    ///
    /// This is done because `pgn_reader`'s `Visitor` trait has a required `end_game`
    /// function, which would ideally return `Game`, but it does not consume the visitor,
    /// so nothing can be moved.
    pub fn into_game(self) -> Result<Game, SanErrorWithMoveNumber> {
        self.result?;

        Ok(Game {
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
            root_variation: Some(self.root_variation),
        })
    }
}

impl pgn_reader::Visitor for Visitor {
    type Result = ();

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        match key.to_ascii_lowercase().as_slice() {
            b"event" => self.event = Some(value.decode_utf8_lossy().to_string()),
            b"site" => self.site = Some(value.decode_utf8_lossy().to_string()),
            b"date" => self.date = Date::from_str(&value.decode_utf8_lossy()).ok(),
            b"white" => self.white = Some(value.decode_utf8_lossy().to_string()),
            b"black" => self.black = Some(value.decode_utf8_lossy().to_string()),
            b"whiteelo" => self.white_elo = value.decode_utf8_lossy().parse().ok(),
            b"blackelo" => self.black_elo = value.decode_utf8_lossy().parse().ok(),
            b"result" => self.outcome = Outcome::from_str(&value.decode_utf8_lossy()).ok(),
            b"round" => self.round = Round::from_str(&value.decode_utf8_lossy()).ok(),
            b"eco" => self.eco = Eco::from_str(&value.decode_utf8_lossy()).ok(),
            b"timecontrol" => self.time_control = Some(value.decode_utf8_lossy().to_string()),
            _ => {},
        }
    }

    fn begin_variation(&mut self) -> Skip {
        if self.result.is_err() {
            return Skip(true);
        }

        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_move_number.index -= 1;
        }

        let current_variation = self.current_variation_tree.last().unwrap_or(&self.root_variation);
        let new_variation = current_variation.new_variation_at_last_move(50);

        self.current_variation_tree.push(new_variation);

        Skip(false)
    }

    fn end_variation(&mut self) {
        if self.result.is_err() {
            return;
        }

        // Remove the current variation because it ended, but get the value of it.
        let Some(current_variation) = self.current_variation_tree.pop() else {
            return;
        };

        let current_variation_move_number = current_variation.move_number();
        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_move_number = MoveNumber { index: current_variation_move_number.index + 1 };
        }

        let current_variation_parent = self.current_variation_tree.last_mut().unwrap_or(&mut self.root_variation);

        current_variation_parent.insert_variation(current_variation);
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.result.is_err() {
            return;
        }

        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_move_number.index += 1;
        }


        let current_variation = self.current_variation_tree.last_mut().unwrap_or(&mut self.root_variation);

        //println!("Current variation position: \n{:?}", current_variation.last_position().board());

        match current_variation.push_move(&san_plus.san) {
            Ok(()) => (),
            //Ok(()) => println!("Move {} is ok", san_plus.san),
            Err(e) => {
                //println!("Move {} is err", san_plus.san);
                self.result = Err(SanErrorWithMoveNumber(e, self.current_move_number));
            }
        }
    }

    fn end_game(&mut self) -> Self::Result {}
}