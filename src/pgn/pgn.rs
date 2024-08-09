use std::fmt::{Display, Formatter, Write};
use std::io::Read;
use pgn_reader::BufferedReader;
use super::visitor::{Visitor, VisitorSanError};
use crate::{Eco, pgn::{Outcome, Date, Round}, Variation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pgn {
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<Date>,
    pub round: Option<Round>,
    pub white: Option<String>,
    pub white_elo: Option<u16>,
    pub black: Option<String>,
    pub black_elo: Option<u16>,
    pub outcome: Option<Outcome>,
    pub eco: Option<Eco>,
    // TODO: Make a time control type
    pub time_control: Option<String>,
    pub root_variation: Option<Variation>,
}

#[derive(Debug)]
pub enum PgnParseError {
    Io(std::io::Error),
    SanError(VisitorSanError)
}

impl Pgn {
    #[allow(clippy::should_implement_trait)]
    /// Reads all games in this string.
    ///
    /// # Errors
    ///
    /// These are errors for every item in the `Vec`. This function does not error itself.
    /// See [`PgnParseError`].
    pub fn from_str(pgn: &str) -> Vec<Result<Self, PgnParseError>> {
        let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);
        
        Self::from_reader(&mut reader)
    }
    
    /// Reads all games in this reader.
    ///
    /// It is guaranteed that the resulting `Vec` will have the same amount of games as the reader does.
    /// Some of them might be errors though.
    /// 
    /// # Errors
    /// 
    /// These are errors for every item in the `Vec`. This function does not error itself.
    /// See [`PgnParseError`].
    pub fn from_reader<R>(reader: &mut BufferedReader<R>) -> Vec<Result<Self, PgnParseError>> where R: Read {
        let mut pgns = Vec::new();

        loop {
            let mut pgn_visitor = Visitor::new();

            let result = reader.read_game(&mut pgn_visitor);

            match result {
                Ok(Some(())) => match pgn_visitor.into_pgn() {
                    Ok(pgn) => pgns.push(Ok(pgn)),
                    Err(e) => pgns.push(Err(PgnParseError::SanError(e))),
                },
                Err(e) => pgns.push(Err(PgnParseError::Io(e))),
                // Empty reader
                Ok(None) => break,
            }
        }

        pgns
    }
}

impl Display for Pgn {
    /// Returns the string representation of this PGN.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! push_pgn_header {
            ($field_name:ident) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", stringify!([<$field_name:camel>]), " \"", $field_name, "\"]\n"))?;
                    }
                }
            };

            ($field_name:ident, $header_title:expr) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", $header_title, " \"", $field_name, "\"]\n"))?;
                    }
                }
            };

            (non_str_display: $field_name:ident) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", stringify!([<$field_name:camel>]), " \"", &$field_name.to_string(), "\"]\n"))?;
                    }
                }
            };

            (non_str_display: $field_name:ident, $header_title:expr) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", $header_title, " \"", &$field_name.to_string(), "\"]\n"))?;
                    }
                }
            };
        }

        push_pgn_header!(event);
        push_pgn_header!(site);
        push_pgn_header!(non_str_display: date);
        push_pgn_header!(non_str_display: round);
        push_pgn_header!(white);
        push_pgn_header!(black);
        push_pgn_header!(non_str_display: outcome, "Result");
        push_pgn_header!(non_str_display: white_elo);
        push_pgn_header!(non_str_display: black_elo);
        push_pgn_header!(non_str_display: eco, "ECO");
        push_pgn_header!(time_control);

        let Some(root_variation) = &self.root_variation else {
            return Ok(());
        };

        f.write_char('\n')?;
        f.write_str(&root_variation.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::unreachable)]
#[allow(clippy::panic)]
mod tests {
    use shakmaty::{Chess, Color};
    use crate::{EcoCategory, MoveNumber, TurnsCapacity, VariationsCapacity};
    use super::*;
    use test_case::test_case;
    use pretty_assertions::assert_eq;
    use std::num::{NonZeroU16, NonZeroU8};
    use shakmaty::san::{San, SanError};
    use crate::variation::play_san_strings;

    const PGN1: &str = r#"[Event "Let's Play!"]
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

1. e4 ( 1. d4 1... d5 ( 1... f5 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. d3"#;

    const PGN2: &str = r#"[Event "Live Chess"]
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

    const PGN3: &str = r#"[Date "????.01.??"]
[Round "1"]
[Result "1/2-1/2"]
[ECO "C50"]

1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf3 ) 2... Nf6 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. d3"#;

    /// Erroneous (1... h4).
    const PGN4: &str = "1. e4 ( 1. d4 1... d5 ( 1... h4 ) ) 1... e5";
    /// Erroneous (4. Nf2).
    const PGN5: &str = "1. e4 ( 1. d4 1... d5 ( 1... f5 2. g3 ( 2. c4 2... Nf6 3. Nc3 3... e6 ( 3... g6 ) 4. Nf2 ) 2... Nf6 ) ) 1... e5";
    /// Erroneous (3. Nd2 is ambiguous).
    const PGN6: &str = "1. Nf3 1... a6 2. d3 2... a5 3. Nd2";
    
    fn pgn1_parsed() -> Pgn {
        let mut root_var = Variation::new(
            Chess::default(),
            TurnsCapacity::default()
        );

        play_san_strings!(root_var,
            "e4",
            "e5",
            "Nf3",
            "Nc6",
            "Bc4",
            "Nf6",
            "d3"
        ).unwrap();

        // CLIPPY: u16 as usize is safe.
        #[allow(clippy::as_conversions)]
        let bc5_var_index = MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(3).unwrap()).index as usize;
        let mut bc5_var = Variation::new(
            root_var.get_position(bc5_var_index).unwrap().into_owned(),
            TurnsCapacity(1)
        );

        play_san_strings!(bc5_var, "Bc5").unwrap();

        let mut d4_var = Variation::new(
            root_var.get_position(0).unwrap().into_owned(),
            TurnsCapacity(2)
        );

        play_san_strings!(d4_var,
            "d4",
            "d5"
        ).unwrap();

        let mut f5_var = Variation::new(
            d4_var.position_before_last_move().into_owned(),
            TurnsCapacity(1)
        );

        play_san_strings!(f5_var, "f5").unwrap();

        d4_var.insert_variation(1, f5_var).unwrap();
        root_var.insert_variation(0, d4_var).unwrap();
        root_var.insert_variation(bc5_var_index, bc5_var).unwrap();

        Pgn {
            event: Some("Let's Play!".to_string()),
            site: Some("Chess.com".to_string()),
            date: Some(Date::new(Some(2024), Some(unsafe { NonZeroU8::new_unchecked(2) }), Some(unsafe { NonZeroU8::new_unchecked(14) })).unwrap()),
            white: Some("4m9n".to_string()),
            black: Some("tigerros0".to_string()),
            outcome: Some(Outcome::Decisive { winner: Color::Black }),
            round: Some(Round::Unknown),
            white_elo: Some(1490),
            black_elo: Some(1565),
            eco: Some(Eco::new(EcoCategory::C, 50).unwrap()),
            time_control: Some("600+0".to_string()),
            root_variation: Some(root_var),
        }
    }

    fn pgn2_parsed() -> Pgn {
        let mut root_var = Variation::new(Chess::new(), TurnsCapacity(4));

        play_san_strings!(
            root_var,
            "g4",
            "e5",
            "f3",
            "Qh4"
        ).unwrap();

        Pgn {
            event: Some("Live Chess".to_string()),
            site: Some("Lichess".to_string()),
            date: Some(Date::new(Some(2024), Some(unsafe { NonZeroU8::new_unchecked(2) }), None).unwrap()),
            white: Some("Nasrin_Babayeva".to_string()),
            white_elo: Some(1765),
            black: Some("tigerros0".to_string()),
            black_elo: Some(1584),
            outcome: Some(Outcome::Decisive { winner: Color::Black }),
            round: Some(Round::Multipart(vec![3, 1, 2])),
            eco: Some(Eco::new(EcoCategory::A, 00).unwrap()),
            time_control: Some("600+2".to_string()),
            root_variation: Some(root_var),
        }
    }

    fn pgn3_parsed() -> Pgn {
        let mut root_var = Variation::new(Chess::new(), TurnsCapacity(1));

        play_san_strings!(root_var,
            "e4",
            "e5",
            "Nf3",
            "Nc6",
            "Bc4",
            "Nf6",
            "d3"
        ).unwrap();

        let mut d4_var = Variation::new(root_var.first_position().clone(), TurnsCapacity(2));

        play_san_strings!(d4_var,
            "d4",
            "d5"
        ).unwrap();

        let mut f5_var = Variation::new(d4_var.position_before_last_move().into_owned(), TurnsCapacity(3));

        play_san_strings!(f5_var,
            "f5",
            "g3",
            "Nf6"
        ).unwrap();

        let c4_var_index = 1;
        let mut c4_var = Variation::new(
            f5_var.get_position(c4_var_index)
                .unwrap()
                .into_owned(),
            TurnsCapacity(5)
        );

        play_san_strings!(c4_var,
            "c4",
            "Nf6",
            "Nc3",
            "e6",
            "Nf3"
        ).unwrap();

        let g6_var_index = 3;
        let mut g6_var = Variation::new(
            c4_var.get_position(g6_var_index)
                .unwrap()
                .into_owned(),
            TurnsCapacity(1)
        );

        play_san_strings!(g6_var, "g6").unwrap();

        // CLIPPY: u16 as usize is safe.
        #[allow(clippy::as_conversions)]
        let bc5_var_index = MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(3).unwrap()).index as usize;
        let mut bc5_var = Variation::new(
            root_var.get_position(bc5_var_index).unwrap().into_owned(),
            TurnsCapacity(1)
        );

        play_san_strings!(bc5_var, "Bc5").unwrap();

        c4_var.insert_variation(g6_var_index, g6_var).unwrap();
        f5_var.insert_variation(c4_var_index, c4_var).unwrap();
        d4_var.insert_variation(1, f5_var).unwrap();
        root_var.insert_variation(0, d4_var).unwrap();
        root_var.insert_variation(bc5_var_index, bc5_var).unwrap();

        Pgn {
            event: None,
            site: None,
            date: Some(Date::new(None, Some(unsafe { NonZeroU8::new_unchecked(1) }), None).unwrap()),
            white: None,
            black: None,
            outcome: Some(Outcome::Draw),
            round: Some(Round::Normal(1)),
            white_elo: None,
            black_elo: None,
            eco: Some(Eco::new(EcoCategory::C, 50).unwrap()),
            time_control: None,
            root_variation: Some(root_var),
        }
    }

    fn pgn4_parsed() -> PgnParseError {
        PgnParseError::SanError(VisitorSanError {
            move_index: 1,
            san: San::from_ascii(b"h4").unwrap(),
            error: SanError::IllegalSan,
        })
    }

    fn pgn5_parsed() -> PgnParseError {
        PgnParseError::SanError(VisitorSanError {
            move_index: 6,
            san: San::from_ascii(b"Nf2").unwrap(),
            error: SanError::IllegalSan
        })
    }

    fn pgn6_parsed() -> PgnParseError {
        PgnParseError::SanError(VisitorSanError {
            move_index: 4,
            san: San::from_ascii(b"Nd2").unwrap(),
            error: SanError::AmbiguousSan
        })
    }

    #[test_case(PGN1, Ok(pgn1_parsed()))]
    #[test_case(PGN2, Ok(pgn2_parsed()))]
    #[test_case(PGN3, Ok(pgn3_parsed()))]
    #[test_case(PGN4, Err(pgn4_parsed()))]
    #[test_case(PGN5, Err(pgn5_parsed()))]
    #[test_case(PGN6, Err(pgn6_parsed()))]
    fn to_pgn_from_pgn(pgn_str: &str, parsed_pgn: Result<Pgn, PgnParseError>) {
        let from_str_vec = Pgn::from_str(pgn_str);
        let from_str = from_str_vec.first().unwrap();

        match parsed_pgn {
            Ok(parsed_pgn) => {
                assert_eq!(from_str.as_ref().unwrap(), &parsed_pgn);
                assert_eq!(parsed_pgn.to_string(), pgn_str);
            }
            Err(e1) => {
                assert!(from_str.is_err());

                let Err(e2) = from_str else {
                    unreachable!();
                };

                // Put `e1` on the right side of the assert because that is the "correct" side.
                match (e1, e2) {
                    (PgnParseError::Io(e1), PgnParseError::Io(e2)) => assert_eq!(e2.to_string(), e1.to_string()),
                    (PgnParseError::SanError(e1), PgnParseError::SanError(e2)) => assert_eq!(e2, &e1),
                    _ => panic!("errors are not the same variant")
                }
            }
        }
    }
}