use crate::{Date, Eco, Outcome, Pgn, RawHeaderOwned, Round};
use shakmaty::fen::{Fen, ParseFenError};
use std::str::FromStr;

/// This is just a [`Pgn`], but with guaranteed "Seven Tag Roster" fields.
/// Create by calling the [`TryFrom`] implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SevenTagRoster<M> {
    /// See "Event" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN158>
    pub event: RawHeaderOwned,
    /// See "Site" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN164>
    pub site: RawHeaderOwned,
    /// See "Date" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN170>
    pub date: Date,
    /// See "Round" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN176>
    pub round: Round,
    /// See "White" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN183>
    pub white: RawHeaderOwned,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.1.2>
    pub white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// See "Black" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN191>
    pub black: RawHeaderOwned,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.1.2>
    pub black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// See "Result" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN197>
    pub outcome: Outcome,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.4.1>
    pub eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    // TODO: Make a time control type
    /// Not typed yet.
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.6>
    pub time_control: Option<RawHeaderOwned>,
    /// Note that this FEN may not be a legal position.
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.7.2>
    pub fen: Option<Result<Fen, ParseFenError>>,
    /// The actual game. See [`Movetext`](crate::Movetext).
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c8.2>
    pub movetext: M,
}

/// Note that multiple fields may be missing/erroneous.
/// In that case, the error will be the first missing/erroneous field, in the order of the variants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    EventIsNone,
    SiteIsNone,
    DateIsNone,
    DateIsErr,
    RoundIsNone,
    RoundIsErr,
    WhiteIsNone,
    BlackIsNone,
    OutcomeIsNone,
    OutcomeIsErr,
}

impl<M> TryFrom<Pgn<M>> for SevenTagRoster<M> {
    type Error = Error;

    fn try_from(pgn: Pgn<M>) -> Result<Self, Self::Error> {
        let Some(event) = pgn.event else {
            return Err(Error::EventIsNone);
        };

        let Some(site) = pgn.site else {
            return Err(Error::SiteIsNone);
        };

        let Some(date) = pgn.date else {
            return Err(Error::DateIsNone);
        };

        let Ok(date) = date else {
            return Err(Error::DateIsErr);
        };

        let Some(round) = pgn.round else {
            return Err(Error::RoundIsNone);
        };

        let Ok(round) = round else {
            return Err(Error::RoundIsErr);
        };

        let Some(white) = pgn.white else {
            return Err(Error::WhiteIsNone);
        };

        let Some(black) = pgn.black else {
            return Err(Error::BlackIsNone);
        };

        let Some(outcome) = pgn.outcome else {
            return Err(Error::OutcomeIsNone);
        };

        let Ok(outcome) = outcome else {
            return Err(Error::OutcomeIsErr);
        };

        Ok(Self {
            event,
            site,
            date,
            round,
            white,
            white_elo: pgn.white_elo,
            black,
            black_elo: pgn.black_elo,
            outcome,
            eco: pgn.eco,
            time_control: pgn.time_control,
            fen: None,
            movetext: pgn.movetext,
        })
    }
}
