use crate::movetext::{Movetext, SimpleMovetext};
use crate::{Date, Eco, Outcome, Pgn, RawHeaderOwned, Round};
use pgn_reader::{Nag, RawHeader, Skip};
use shakmaty::fen::{Fen, ParseFenError};
use shakmaty::san::SanPlus;
use std::borrow::Cow;
use std::str::{FromStr, Utf8Error};

pub(super) struct Visitor<M>
where
    M: Movetext,
{
    event: Option<RawHeaderOwned>,
    site: Option<RawHeaderOwned>,
    date: Option<Result<Date, <Date as FromStr>::Err>>,
    round: Option<Result<Round, <Round as FromStr>::Err>>,
    white: Option<RawHeaderOwned>,
    white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    black: Option<RawHeaderOwned>,
    black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    outcome: Option<Result<Outcome, <Outcome as FromStr>::Err>>,
    eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    time_control: Option<RawHeaderOwned>,
    fen: Option<Result<Fen, ParseFenError>>,
    movetext: Option<M>,
}

impl<M> Visitor<M>
where
    M: Movetext,
{
    pub const fn new() -> Self {
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
            movetext: None,
        }
    }

    /// Moves relevant contents of the visitor into a new [`Pgn`].
    ///
    /// Call this after you visit [`Visitor`] with a reader.
    ///
    /// This is done because `pgn_reader`'s `Visitor` trait has a required `end_game`
    /// function, which would ideally return [`Pgn`], but it does not consume the visitor,
    /// so nothing can be moved.
    pub fn into_pgn(self) -> Pgn<M> {
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
            movetext: self.movetext.map(Movetext::output),
        }
    }
}

impl<M> pgn_reader::Visitor for Visitor<M>
where
    M: Movetext,
{
    type Result = ();

    fn header(&mut self, key: &[u8], raw_header: RawHeader<'_>) {
        match key.to_ascii_lowercase().as_slice() {
            b"event" => self.event = Some(raw_header.into()),
            b"site" => self.site = Some(raw_header.into()),
            b"date" => self.date = Some(Date::from_str(&raw_header.decode_utf8_lossy())),
            b"white" => self.white = Some(raw_header.into()),
            b"black" => self.black = Some(raw_header.into()),
            b"whiteelo" => self.white_elo = Some(raw_header.decode_utf8_lossy().parse()),
            b"blackelo" => self.black_elo = Some(raw_header.decode_utf8_lossy().parse()),
            b"result" => self.outcome = Some(Outcome::from_str(&raw_header.decode_utf8_lossy())),
            b"round" => self.round = Some(Round::from_str(&raw_header.decode_utf8_lossy())),
            b"eco" => self.eco = Some(Eco::from_str(&raw_header.decode_utf8_lossy())),
            b"timecontrol" => self.time_control = Some(raw_header.into()),
            b"fen" => self.fen = Some(Fen::from_str(&raw_header.decode_utf8_lossy())),
            _ => {}
        }
    }

    fn begin_game(&mut self) {
        self.movetext = Some(M::begin_game());
    }

    fn begin_variation(&mut self) -> Skip {
        self.movetext
            .as_mut()
            .map_or(Skip(true), Movetext::begin_variation)
    }

    fn end_variation(&mut self) {
        if let Some(movetext) = &mut self.movetext {
            movetext.end_variation();
        }
    }

    fn san(&mut self, san_plus: SanPlus) {
        if let Some(movetext) = &mut self.movetext {
            movetext.san(san_plus);
        }
    }

    fn end_game(&mut self) -> Self::Result {
        if let Some(movetext) = &mut self.movetext {
            movetext.end_game();
        }
    }
}
