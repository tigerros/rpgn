// CLIPPY: These samples are only used in tests and benches, any panics will be caught.
#![allow(clippy::unwrap_used)]
#![allow(clippy::unreachable)]
#![allow(clippy::missing_panics_doc)]

use crate::movetext::{Sans, Variation};
#[cfg(test)]
use crate::Movetext;
use crate::RawHeaderOwned;
use crate::{sans, variation};
use crate::{Date, Eco, EcoCategory, Outcome, Pgn, Round};
use deranged::{OptionRangedU16, OptionRangedU8, RangedU16, RangedU8};
use pgn_reader::RawHeader;
#[cfg(test)]
use pretty_assertions::assert_eq;
use shakmaty::fen::Fen;
use shakmaty::san::SanPlus;
use shakmaty::Color;
use std::fmt::Debug;
#[cfg(test)]
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct PgnSample<M> {
    pub string: &'static str,
    pub parsed: Result<Pgn<M>, io::Error>,
}

impl<M> PgnSample<M> {
    pub const fn new(string: &'static str, parsed: Result<Pgn<M>, io::Error>) -> Self {
        Self { string, parsed }
    }
}

#[cfg(test)]
impl<M> PgnSample<M>
where
    M: Movetext + PartialEq + Debug + Display,
{
    pub fn test(&self) {
        let from_str_vec = Pgn::from_str(self.string);
        let from_str = from_str_vec.first().unwrap();

        match &self.parsed {
            Ok(parsed_pgn) => {
                assert_eq!(from_str.as_ref().unwrap(), parsed_pgn);
                assert_eq!(parsed_pgn.to_string(), self.string);
            }
            Err(e1) => {
                assert!(from_str.is_err());

                let Err(e2) = from_str else {
                    unreachable!();
                };

                // Put `e1` on the right side of the assert because that is the "correct" side.
                assert_eq!(e2.to_string(), e1.to_string());
            }
        }
    }
}

pub fn variation0() -> PgnSample<Variation<SanPlus>> {
    const PGN: &str = r#"[Event "Let's Play!"]
[Site "Chess.com"]
[Date "0000.02.14"]
[Round "?"]
[White "4m9n"]
[Black "tigerros0"]
[Result "0-1"]
[WhiteElo "1490"]
[BlackElo "1565"]
[ECO "C50"]
[TimeControl "600+0"]

1. e4 ( 1. d4 1... d5 ( 1... f5 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. Nc3"#;

    let movetext = variation! {
        (b"e4", [{ b"d4", (b"d5", [{ b"f5" }]) }]),
        b"e5",
        b"Nf3",
        b"Nc6",
        b"Bc4",
        (b"Nf6", [{ b"Bc5" }]),
        b"Nc3"
    };

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: Some(RawHeaderOwned::from(RawHeader(b"Let's Play!"))),
            site: Some(RawHeaderOwned::from(RawHeader(b"Chess.com"))),
            date: Some(Ok(Date {
                year: OptionRangedU16::Some(RangedU16::new_static::<0>()),
                month: OptionRangedU8::Some(RangedU8::new_static::<2>()),
                day: OptionRangedU8::Some(RangedU8::new_static::<14>()),
            })),
            white: Some(RawHeaderOwned::from(RawHeader(b"4m9n"))),
            black: Some(RawHeaderOwned::from(RawHeader(b"tigerros0"))),
            outcome: Some(Ok(Outcome::Decisive {
                winner: Color::Black,
            })),
            round: Some(Ok(Round::Unknown)),
            white_elo: Some(Ok(1490)),
            black_elo: Some(Ok(1565)),
            eco: Some(Ok(Eco {
                category: EcoCategory::C,
                subcategory: RangedU8::new_static::<50>(),
            })),
            time_control: Some(RawHeaderOwned::from(RawHeader(b"600+0"))),
            fen: None,
            movetext,
        }),
    )
}

pub fn variation1() -> PgnSample<Variation<SanPlus>> {
    const PGN: &str = r#"[Event "Live Chess"]
[Site "Lichess"]
[Date "9999.02.??"]
[Round "3.1.2"]
[White "Nasrin_Babayeva"]
[Black "tigerros0"]
[Result "0-1"]
[WhiteElo "1765"]
[BlackElo "1584"]
[ECO "A00"]
[TimeControl "600+2"]

1. g4 1... e5 2. f3 2... Qh4#"#;

    let movetext = variation! { b"g4", b"e5", b"f3", b"Qh4#" };

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: Some(RawHeaderOwned::from(RawHeader(b"Live Chess"))),
            site: Some(RawHeaderOwned::from(RawHeader(b"Lichess"))),
            date: Some(Ok(Date {
                year: OptionRangedU16::Some(RangedU16::new_static::<9999>()),
                month: OptionRangedU8::Some(RangedU8::new_static::<2>()),
                day: OptionRangedU8::None,
            })),
            white: Some(RawHeaderOwned::from(RawHeader(b"Nasrin_Babayeva"))),
            white_elo: Some(Ok(1765)),
            black: Some(RawHeaderOwned::from(RawHeader(b"tigerros0"))),
            black_elo: Some(Ok(1584)),
            outcome: Some(Ok(Outcome::Decisive {
                winner: Color::Black,
            })),
            round: Some(Ok(Round::Multipart(vec![3, 1, 2]))),
            eco: Some(Ok(Eco {
                category: EcoCategory::A,
                subcategory: RangedU8::new_static::<0>(),
            })),
            time_control: Some(RawHeaderOwned::from(RawHeader(b"600+2"))),
            fen: None,
            movetext,
        }),
    )
}

pub fn variation2() -> PgnSample<Variation<SanPlus>> {
    const PGN: &str = r#"[Date "????.01.??"]
[Round "1"]
[Result "1/2-1/2"]
[ECO "C50"]

1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf3 ) 2... Nf6 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) ( 3... Nge7 ) 4. d3 ( 4. O-O )"#;

    let movetext = variation! {
        (b"e4", [{ b"d4", (b"d5", [{ b"f5", (b"g3", [{ b"c4", b"Nf6", b"Nc3", (b"e6", [{ b"g6" }]), b"Nf3" }]), b"Nf6" }]) }]),
        b"e5",
        b"Nf3",
        b"Nc6",
        b"Bc4",
        (b"Nf6", [{ b"Bc5" }, { b"Nge7" }]),
        (b"d3", [{ b"O-O" }])
    };

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: None,
            site: None,
            date: Some(Ok(Date {
                year: OptionRangedU16::None,
                month: OptionRangedU8::Some(RangedU8::new_static::<1>()),
                day: OptionRangedU8::None,
            })),
            white: None,
            black: None,
            outcome: Some(Ok(Outcome::Draw)),
            round: Some(Ok(Round::Normal(1))),
            white_elo: None,
            black_elo: None,
            eco: Some(Ok(Eco {
                category: EcoCategory::C,
                subcategory: RangedU8::new_static::<50>(),
            })),
            time_control: None,
            fen: None,
            movetext,
        }),
    )
}

/// Nd2 is ambiguous, but we don't care.
pub fn sans0() -> PgnSample<Sans<SanPlus>> {
    const PGN: &str = r#"[FEN "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"]

1. Nf3 1... a6 2. d3 2... a5 3. Nd2"#;

    let movetext = sans!(b"Nf3", b"a6", b"d3", b"a5", b"Nd2");

    PgnSample::new(
        PGN,
        Ok(Pgn {
            fen: Some(Ok(Fen::from_ascii(
                b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            )
            .unwrap())),
            movetext,
            ..Default::default()
        }),
    )
}

/// One move.
pub fn sans1() -> PgnSample<Sans<SanPlus>> {
    const PGN: &str = "1. e4";

    let movetext = sans!(b"e4");

    PgnSample::new(
        PGN,
        Ok(Pgn {
            movetext,
            ..Default::default()
        }),
    )
}
