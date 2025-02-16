//! These are samples I use in tests and benchmarks.

// CLIPPY: These samples are only used in tests and benches, any panics will be caught.
#![allow(clippy::unwrap_used)]
#![allow(clippy::unreachable)]
#![allow(clippy::missing_panics_doc)]

use crate::movetext::SimpleMovetext;
use crate::{Date, Eco, EcoCategory, Outcome, Pgn, Round};
use crate::{Movetext, RawHeaderOwned, VariationMovetext};
use pgn_reader::RawHeader;
use shakmaty::san::SanPlus;
use shakmaty::{Color, Move};
use std::fmt::{Debug, Display};
use std::io;
use std::num::NonZeroU8;

macro_rules! san {
    ($san:literal) => {
        SanPlus::from_ascii($san).unwrap()
    };
}

macro_rules! variation_movetext {
    (_turn: $san:literal) => {
        (san!($san), vec![])
    };
    (_turn: ($san:literal, [$($vars:tt),+])) => {
        (san!($san), vec![$(variation_movetext! $vars),+])
    };
    {$($turn:tt),+} => {
        VariationMovetext(vec![$(variation_movetext!(_turn: $turn)),+])
    };
}

macro_rules! simple_movetext {
    ($($san:literal),+) => {
        SimpleMovetext(vec![$(san!($san)),+])
    };
}

#[derive(Debug)]
pub struct PgnSample<M>
where
    M: Movetext<Output: Debug>,
{
    pub string: &'static str,
    pub parsed: Result<Pgn<M>, io::Error>,
}

impl<M> PgnSample<M>
where
    M: Movetext<Output: Debug>,
{
    pub const fn new(string: &'static str, parsed: Result<Pgn<M>, io::Error>) -> Self {
        Self { string, parsed }
    }
}

pub fn sample0() -> PgnSample<VariationMovetext> {
    const PGN: &str = r#"[Event "Let's Play!"]
[Site "Chess.com"]
[Date "2024.02.14"]
[Round "?"]
[White "4m9n"]
[Black "tigerros0"]
[Result "0-1"]
[WhiteElo "1490"]
[BlackElo "1565"]
[ECO "C50"]
[TimeControl "600+0"]

1. e4 ( 1. d4 1... d5 ( 1... f5 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. Nc3"#;

    let movetext = variation_movetext! {
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
            date: Some(Ok(Date::new(
                Some(2024),
                Some(unsafe { NonZeroU8::new_unchecked(2) }),
                Some(unsafe { NonZeroU8::new_unchecked(14) }),
            )
            .unwrap())),
            white: Some(RawHeaderOwned::from(RawHeader(b"4m9n"))),
            black: Some(RawHeaderOwned::from(RawHeader(b"tigerros0"))),
            outcome: Some(Ok(Outcome::Decisive {
                winner: Color::Black,
            })),
            round: Some(Ok(Round::Unknown)),
            white_elo: Some(Ok(1490)),
            black_elo: Some(Ok(1565)),
            eco: Some(Ok(Eco::new(EcoCategory::C, 50).unwrap())),
            time_control: Some(RawHeaderOwned::from(RawHeader(b"600+0"))),
            fen: None,
            movetext: Some(movetext),
        }),
    )
}

pub fn simple_sample1() -> PgnSample<SimpleMovetext> {
    const PGN: &str = r#"[Event "Live Chess"]
[Site "Lichess"]
[Date "2024.02.??"]
[Round "3.1.2"]
[White "Nasrin_Babayeva"]
[Black "tigerros0"]
[Result "0-1"]
[WhiteElo "1765"]
[BlackElo "1584"]
[ECO "A00"]
[TimeControl "600+2"]

1. g4 1... e5 2. f3 2... Qh4#"#;

    let movetext = simple_movetext!(b"g4", b"e5", b"f3", b"Qh4");

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: Some(RawHeaderOwned::from(RawHeader(b"Live Chess"))),
            site: Some(RawHeaderOwned::from(RawHeader(b"Lichess"))),
            date: Some(Ok(Date::new(
                Some(2024),
                Some(unsafe { NonZeroU8::new_unchecked(2) }),
                None,
            )
            .unwrap())),
            white: Some(RawHeaderOwned::from(RawHeader(b"Nasrin_Babayeva"))),
            white_elo: Some(Ok(1765)),
            black: Some(RawHeaderOwned::from(RawHeader(b"tigerros0"))),
            black_elo: Some(Ok(1584)),
            outcome: Some(Ok(Outcome::Decisive {
                winner: Color::Black,
            })),
            round: Some(Ok(Round::Multipart(vec![3, 1, 2]))),
            eco: Some(Ok(Eco::new(EcoCategory::A, 00).unwrap())),
            time_control: Some(RawHeaderOwned::from(RawHeader(b"600+2"))),
            fen: None,
            movetext: Some(movetext),
        }),
    )
}

pub fn simple_sample2() -> PgnSample<SimpleMovetext> {
    const PGN: &str = r#"[Date "????.01.??"]
[Round "1"]
[Result "1/2-1/2"]
[ECO "C50"]

1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf3 ) 2... Nf6 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. d3"#;

    let movetext = simple_movetext!(b"e4", b"e5", b"Nf3", b"Nc6", b"Bc4", b"Nf6", b"d3");

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: None,
            site: None,
            date: Some(Ok(Date::new(
                None,
                Some(unsafe { NonZeroU8::new_unchecked(1) }),
                None,
            )
            .unwrap())),
            white: None,
            black: None,
            outcome: Some(Ok(Outcome::Draw)),
            round: Some(Ok(Round::Normal(1))),
            white_elo: None,
            black_elo: None,
            eco: Some(Ok(Eco::new(EcoCategory::C, 50).unwrap())),
            time_control: None,
            fen: None,
            movetext: Some(movetext),
        }),
    )
}

/// Erroneous (3. Nd2 is ambiguous). We don't care though.
pub fn simple_sample5() -> PgnSample<SimpleMovetext> {
    const PGN: &str = "1. Nf3 1... a6 2. d3 2... a5 3. Nd2";

    let movetext = simple_movetext!(b"Nf3", b"a6", b"d3", b"a5", b"Nd2");

    PgnSample::new(
        PGN,
        Ok(Pgn {
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
            movetext: Some(movetext),
        }),
    )
}

pub fn pgn_sample6() -> PgnSample<SimpleMovetext> {
    const PGN: &str = "1. e4";

    let movetext = simple_movetext!(b"e4");

    PgnSample::new(
        PGN,
        Ok(Pgn {
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
            movetext: Some(movetext),
        }),
    )
}
