use std::borrow::Cow;
use pgn_reader::{RawHeader, Skip};
use std::str::{FromStr, Utf8Error};
use shakmaty::{CastlingMode, Chess, Position, PositionError};
use shakmaty::fen::{Fen, ParseFenError};
use shakmaty::san::SanPlus;
use crate::{Eco, pgn::{Date, Round, Outcome, Pgn}};
use crate::san_list::SanList;

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
    san_list: SanList,
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
            san_list: SanList(Vec::with_capacity(0)),
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
            san_list: self.san_list,
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
    
    fn san(&mut self, san_plus: SanPlus) {
        self.san_list.0.push(san_plus);
    }

    fn end_game(&mut self) -> Self::Result {}
}