use std::fmt::{Display, Formatter, Write};
use std::io::Read;
use pgn_reader::BufferedReader;
use super::visitor::Visitor;
use crate::{Eco, pgn::{Outcome, Date, Round}, Variation, VariationSanPlayError};

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
    SanError(VariationSanPlayError)
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
                match (e1, e2) {
                    (PgnParseError::Io(e1), PgnParseError::Io(e2)) => assert_eq!(e2.to_string(), e1.to_string()),
                    (PgnParseError::SanError(e1), PgnParseError::SanError(e2)) => assert_eq!(e2, &e1),
                    _ => panic!("errors are not the same variant")
                }
            }
        }
    }
}