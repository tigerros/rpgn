use pgn_reader::{RawHeader, Skip};
use std::str::FromStr;
use shakmaty::{Chess, Position};
use shakmaty::san::SanPlus;
use crate::{Variation, Eco, pgn::{Date, Round, Outcome, Pgn}, TurnsCapacity, VariationsCapacity, VariationSanPlayError};

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
    variation_tree: Vec<(usize, Variation)>,
    current_turn_index: usize,
    root_variation: Variation,
    result: Result<(), VariationSanPlayError>
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
            variation_tree: Vec::with_capacity(0),
            current_turn_index: 0,
            root_variation: Variation::new(Chess::default(), TurnsCapacity::default()),
            result: Ok(())
        }
    }

    /// Moves relevant contents of the visitor into a new [`Pgn`].
    ///
    /// Call this after you visit [`Visitor`] with a reader.
    ///
    /// This is done because `pgn_reader`'s `Visitor` trait has a required `end_game`
    /// function, which would ideally return [`Pgn`], but it does not consume the visitor,
    /// so nothing can be moved.
    pub fn into_pgn(self) -> Result<Pgn, VariationSanPlayError> {
        self.result?;

        Ok(Pgn {
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

        // CLIPPY: There's never going to be usize::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_turn_index -= 1;
        }

        let current_variation = self.variation_tree.last().map_or(&self.root_variation, |pair| &pair.1);

        println!("Beginning var: {:?}", current_variation.position_before_last_move().board());
        let new_variation = Variation::new(current_variation.position_before_last_move().clone(), TurnsCapacity(50));

        self.variation_tree.push((self.current_turn_index, new_variation));

        Skip(false)
    }

    fn end_variation(&mut self) {
        if self.result.is_err() {
            return;
        }

        // Remove the current variation because it ended, but get the value of it to push to the parent.
        let Some((ending_variation_move_number, ending_variation)) = self.variation_tree.pop() else {
            return;
        };

        let ending_variation_parent = self.variation_tree.last_mut().map_or(&mut self.root_variation, |pair| &mut pair.1);

        // CLIPPY: There's never going to be u16::MAX moves.
        #[allow(clippy::arithmetic_side_effects)]
        {
            self.current_turn_index = ending_variation_move_number + 1;
        }
        
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
        if self.result.is_err() {
            return;
        }

        let current_variation = self.variation_tree.last_mut().map(|pair| &mut pair.1).unwrap_or(&mut self.root_variation);

        //println!("Current variation position: \n{:?}", current_variation.last_position().board());

        if let Err(error) = current_variation.play_san(&san_plus.san, VariationsCapacity::default()) {
            //println!("Move {} is err", san_plus.san);
            self.result = Err(VariationSanPlayError {
                turn_index: self.current_turn_index,
                //position: current_variation.last_position().into_owned(),
                san: san_plus.san,
                error: error.error,
            });
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