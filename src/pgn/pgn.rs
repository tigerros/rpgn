use std::env::var;
use std::fmt::{Display, Formatter, Write};
use std::io::Read;
use std::str::FromStr;
use pgn_reader::BufferedReader;
use shakmaty::fen::{Fen, ParseFenError};
use super::visitor::{Visitor, RawOwnedHeader};
use crate::{Eco, pgn::{Outcome, Date, Round}};
use crate::concat_strings::concat_strings;
use crate::san_list::SanList;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pgn {
    pub event: Option<RawOwnedHeader>,
    pub site: Option<RawOwnedHeader>,
    pub date: Option<Result<Date, <Date as FromStr>::Err>>,
    pub round: Option<Result<Round, <Round as FromStr>::Err>>,
    pub white: Option<RawOwnedHeader>,
    pub white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    pub black: Option<RawOwnedHeader>,
    pub black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// Called "Result" in the PGN standard.
    pub outcome: Option<Result<Outcome, <Outcome as FromStr>::Err>>,
    pub eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    // TODO: Make a time control type
    pub time_control: Option<RawOwnedHeader>,
    /// Note that this FEN may not be a legal position.
    pub fen: Option<Result<Fen, ParseFenError>>,
    pub san_list: SanList,
}

impl Pgn {
    #[allow(clippy::should_implement_trait)]
    /// Reads all games in this string.
    ///
    /// # Errors
    ///
    /// These are errors for every item in the `Vec`. This function does not error itself.
    /// See [`PgnParseError`].
    pub fn from_str(pgn: &str) -> Vec<Result<Self, std::io::Error>> {
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
    pub fn from_reader<R>(reader: &mut BufferedReader<R>) -> Vec<Result<Self, std::io::Error>> where R: Read {
        let mut pgns = Vec::new();

        loop {
            let mut pgn_visitor = Visitor::new();

            let result = reader.read_game(&mut pgn_visitor);

            match result {
                Ok(Some(())) => pgns.push(Ok(pgn_visitor.into_pgn())),
                Err(e) => pgns.push(Err(e)),
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
            ($field_name:ident, $header_title:expr) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", $header_title, " \"", &$field_name.decode_utf8_lossy(), "\"]\n"))?;
                    }
                }
            };

            (custom_type: $field_name:ident, $header_title:expr) => {
                if let Some(Ok($field_name)) = &self.$field_name {
                    paste::paste! {
                        f.write_str(&crate::concat_strings!("[", $header_title, " \"", &$field_name.to_string(), "\"]\n"))?;
                    }
                }
            };
        }

        push_pgn_header!(event, "Event");
        push_pgn_header!(site, "Site");
        push_pgn_header!(custom_type: date, "Date");
        push_pgn_header!(custom_type: round, "Round");
        push_pgn_header!(white, "White");
        push_pgn_header!(black, "Black");
        push_pgn_header!(custom_type: outcome, "Result");
        push_pgn_header!(custom_type: white_elo, "WhiteElo");
        push_pgn_header!(custom_type: black_elo, "BlackElo");
        push_pgn_header!(custom_type: eco, "ECO");
        push_pgn_header!(time_control, "TimeControl");

        if self.san_list.0.is_empty() {
            return Ok(());
        }

        f.write_char('\n')?;
        self.san_list.fmt(f)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::unreachable)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use test_case::test_case;
    use pretty_assertions::assert_eq;
    use crate::samples::*;

    #[test_case(pgn_sample0())]
    #[test_case(pgn_sample1())]
    #[test_case(pgn_sample2())]
    #[test_case(pgn_sample3())]
    #[test_case(pgn_sample4())]
    #[test_case(pgn_sample5())]
    fn to_pgn_from_pgn(sample: PgnSample) {
        let from_str_vec = Pgn::from_str(sample.string);
        let from_str = from_str_vec.first().unwrap();

        match sample.parsed {
            Ok(parsed_pgn) => {
                assert_eq!(from_str.as_ref().unwrap(), &parsed_pgn);
                assert_eq!(parsed_pgn.to_string(), sample.string);
            }
            Err(e1) => {
                assert!(from_str.is_err());

                let Err(e2) = from_str else {
                    unreachable!();
                };

                // Put `e1` on the right side of the assert because that is the "correct" side.
                assert_eq!(e2.to_string(), e1.to_string())
            }
        }
    }
}