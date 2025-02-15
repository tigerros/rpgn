//! These are samples I use in tests and benchmarks.

// CLIPPY: These samples are used in tests, any panics will be caught.
#![allow(clippy::unwrap_used)]
#![allow(clippy::unreachable)]
#![allow(clippy::missing_panics_doc)]
use crate::{
    pgn::{Date, Outcome, Round},
    Eco, EcoCategory, MoveNumber, Pgn,
};
use shakmaty::san::{San, SanError, SanPlus};
use shakmaty::{Chess, Color};
use std::num::{NonZeroU8, NonZeroUsize};
use std::io;
use crate::san_list::SanList;

macro_rules! san_list {
    ($($san:literal),+) => {
        #[allow(clippy::unwrap_used)]
        { SanList(vec![$(SanPlus::from_ascii($san).unwrap()),+]) }
    };
}

#[derive(Debug)]
pub struct PgnSample {
    pub string: &'static str,
    pub parsed: Result<Pgn, io::Error>,
}

impl PgnSample {
    pub const fn new(string: &'static str, parsed: Result<Pgn, io::Error>) -> Self {
        Self { string, parsed }
    }
}

pub fn variation_sample_fns() -> [fn() -> LegalVariation; 4] {
    [
        variation_sample0,
        variation_sample1,
        variation_sample2,
        variation_sample6,
    ]
}

pub fn pgn_samples() -> [PgnSample; 6] {
    [
        pgn_sample0(),
        pgn_sample1(),
        pgn_sample2(),
        pgn_sample3(),
        pgn_sample4(),
        pgn_sample5(),
    ]
}

pub fn variation_sample0() -> LegalVariation {
    let mut root_var = LegalVariation::new(Chess::default(), TurnsCapacity::default());

    play_san_strings!(root_var, "e4", "e5", "Nf3", "Nc6", "Bc4", "Nf6", "Nc3").unwrap();

    // CLIPPY: u16 as usize is safe.
    #[allow(clippy::as_conversions)]
    let bc5_var_index =
        MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(3).unwrap()).0;
    let mut bc5_var = LegalVariation::new(
        root_var.get_position(bc5_var_index).unwrap().clone(),
        TurnsCapacity(1),
    );

    play_san_strings!(bc5_var, "Bc5").unwrap();

    let mut d4_var = LegalVariation::new(root_var.get_position(0).unwrap().clone(), TurnsCapacity(2));

    play_san_strings!(d4_var, "d4", "d5").unwrap();

    let mut f5_var = LegalVariation::new(d4_var.position_before_last_move().clone(), TurnsCapacity(1));

    play_san_strings!(f5_var, "f5").unwrap();

    d4_var.insert_variation(1, f5_var).unwrap();
    root_var.insert_variation(0, d4_var).unwrap();
    root_var.insert_variation(bc5_var_index, bc5_var).unwrap();

    root_var
}

pub fn pgn_sample0() -> PgnSample {
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

    let san_list = san_list!("e4", "e5", "Nf3", "Nc6", "Bc4", "Nf6", "Nc3");

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: Some("Let's Play!".to_string()),
            site: Some("Chess.com".to_string()),
            date: Some(
                Date::new(
                    Some(2024),
                    Some(unsafe { NonZeroU8::new_unchecked(2) }),
                    Some(unsafe { NonZeroU8::new_unchecked(14) }),
                )
                .unwrap(),
            ),
            white: Some("4m9n".to_string()),
            black: Some("tigerros0".to_string()),
            outcome: Some(Outcome::Decisive {
                winner: Color::Black,
            }),
            round: Some(Round::Unknown),
            white_elo: Some(1490),
            black_elo: Some(1565),
            eco: Some(Eco::new(EcoCategory::C, 50).unwrap()),
            time_control: Some("600+0".to_string()),
            san_list: Some(variation_sample0()),
        }),
    )
}

/// No subvariations.
pub fn variation_sample1() -> LegalVariation {
    let mut root_var = LegalVariation::new(Chess::new(), TurnsCapacity(4));

    play_san_strings!(root_var, "g4", "e5", "f3", "Qh4").unwrap();

    root_var
}

pub fn pgn_sample1() -> PgnSample {
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

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: Some("Live Chess".to_string()),
            site: Some("Lichess".to_string()),
            date: Some(
                Date::new(
                    Some(2024),
                    Some(unsafe { NonZeroU8::new_unchecked(2) }),
                    None,
                )
                .unwrap(),
            ),
            white: Some("Nasrin_Babayeva".to_string()),
            white_elo: Some(1765),
            black: Some("tigerros0".to_string()),
            black_elo: Some(1584),
            outcome: Some(Outcome::Decisive {
                winner: Color::Black,
            }),
            round: Some(Round::Multipart(vec![3, 1, 2])),
            eco: Some(Eco::new(EcoCategory::A, 00).unwrap()),
            time_control: Some("600+2".to_string()),
            san_list: Some(variation_sample1()),
        }),
    )
}

pub fn variation_sample2() -> LegalVariation {
    let mut root_var = LegalVariation::new(Chess::new(), TurnsCapacity(1));

    play_san_strings!(root_var, "e4", "e5", "Nf3", "Nc6", "Bc4", "Nf6", "d3").unwrap();

    let mut d4_var = LegalVariation::new(root_var.first_position().clone(), TurnsCapacity(2));

    play_san_strings!(d4_var, "d4", "d5").unwrap();

    let mut f5_var = LegalVariation::new(d4_var.position_before_last_move().clone(), TurnsCapacity(3));

    play_san_strings!(f5_var, "f5", "g3", "Nf6").unwrap();

    let c4_var_index = 1;
    let mut c4_var = LegalVariation::new(
        f5_var.get_position(c4_var_index).unwrap().clone(),
        TurnsCapacity(5),
    );

    play_san_strings!(c4_var, "c4", "Nf6", "Nc3", "e6", "Nf3").unwrap();

    let g6_var_index = 3;
    let mut g6_var = LegalVariation::new(
        c4_var.get_position(g6_var_index).unwrap().clone(),
        TurnsCapacity(1),
    );

    play_san_strings!(g6_var, "g6").unwrap();

    // CLIPPY: u16 as usize is safe.
    #[allow(clippy::as_conversions)]
    let bc5_var_index =
        MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(3).unwrap()).0;
    let mut bc5_var = LegalVariation::new(
        root_var.get_position(bc5_var_index).unwrap().clone(),
        TurnsCapacity(1),
    );

    play_san_strings!(bc5_var, "Bc5").unwrap();

    c4_var.insert_variation(g6_var_index, g6_var).unwrap();
    f5_var.insert_variation(c4_var_index, c4_var).unwrap();
    d4_var.insert_variation(1, f5_var).unwrap();
    root_var.insert_variation(0, d4_var).unwrap();
    root_var.insert_variation(bc5_var_index, bc5_var).unwrap();

    root_var
}

pub fn pgn_sample2() -> PgnSample {
    const PGN: &str = r#"[Date "????.01.??"]
[Round "1"]
[Result "1/2-1/2"]
[ECO "C50"]

1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf3 ) 2... Nf6 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. d3"#;

    PgnSample::new(
        PGN,
        Ok(Pgn {
            event: None,
            site: None,
            date: Some(
                Date::new(None, Some(unsafe { NonZeroU8::new_unchecked(1) }), None).unwrap(),
            ),
            white: None,
            black: None,
            outcome: Some(Outcome::Draw),
            round: Some(Round::Normal(1)),
            white_elo: None,
            black_elo: None,
            eco: Some(Eco::new(EcoCategory::C, 50).unwrap()),
            time_control: None,
            san_list: Some(variation_sample2()),
        }),
    )
}

/// Erroneous (1... h4).
pub fn pgn_sample3() -> PgnSample {
    const PGN: &str = "1. e4 ( 1. d4 1... d5 ( 1... h4 ) ) 1... e5 2. Nc3";

    PgnSample::new(
        PGN,
        Err(PgnParseError::PgnError(PgnError::SanPlayError(
            VariationSanPlayError {
                turn_index: 1,
                san: San::from_ascii(b"h4").unwrap(),
                error: SanError::IllegalSan,
            },
        ))),
    )
}

/// Erroneous (4. Nf2).
pub fn pgn_sample4() -> PgnSample {
    const PGN: &str = "1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf2 ) 2... Nf6 ) ) 1... e5";

    PgnSample::new(
        PGN,
        Err(PgnParseError::PgnError(PgnError::SanPlayError(
            VariationSanPlayError {
                turn_index: 6,
                san: San::from_ascii(b"Nf2").unwrap(),
                error: SanError::IllegalSan,
            },
        ))),
    )
}

/// Erroneous (3. Nd2 is ambiguous).
pub fn pgn_sample5() -> PgnSample {
    const PGN: &str = "1. Nf3 1... a6 2. d3 2... a5 3. Nd2";

    PgnSample::new(
        PGN,
        Err(PgnParseError::PgnError(PgnError::SanPlayError(
            VariationSanPlayError {
                turn_index: 4,
                san: San::from_ascii(b"Nd2").unwrap(),
                error: SanError::AmbiguousSan,
            },
        ))),
    )
}

/// One move only.
pub fn variation_sample6() -> LegalVariation {
    let mut root_var = LegalVariation::new(Chess::new(), TurnsCapacity(1));

    play_san_strings!(root_var, "e4").unwrap();

    root_var
}

pub fn pgn_sample6() -> PgnSample {
    const PGN: &str = "1. e4";

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
            san_list: Some(variation_sample6()),
        }),
    )
}
