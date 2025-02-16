use std::fmt::{Display, Formatter, Write};
use std::io::{Cursor, Read};
use std::str::FromStr;
use pgn_reader::BufferedReader;
use shakmaty::fen::{Fen, ParseFenError};
use super::visitor::{Visitor};
use crate::{Eco, Outcome, Date, Round, RawHeaderOwned};
use crate::movetext::Movetext;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pgn<O> {
    pub event: Option<RawHeaderOwned>,
    pub site: Option<RawHeaderOwned>,
    pub date: Option<Result<Date, <Date as FromStr>::Err>>,
    pub round: Option<Result<Round, <Round as FromStr>::Err>>,
    pub white: Option<RawHeaderOwned>,
    pub white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    pub black: Option<RawHeaderOwned>,
    pub black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// Called "Result" in the PGN standard.
    pub outcome: Option<Result<Outcome, <Outcome as FromStr>::Err>>,
    pub eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    // TODO: Make a time control type
    pub time_control: Option<RawHeaderOwned>,
    /// Note that this FEN may not be a legal position.
    pub fen: Option<Result<Fen, ParseFenError>>,
    pub movetext: Option<O>,
}

impl<O> Default for Pgn<O> {
    /// Creates a [`Pgn`] with all fields set to [`None`].
    fn default() -> Self {
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
}

impl<O> Pgn<O> {
    #[allow(clippy::should_implement_trait)]
    /// Reads all games in this string.
    ///
    /// # Errors
    /// See [`pgn_reader::BufferedReader::read_game`].
    pub fn from_str<M>(pgn: &str) -> Vec<Result<Self, std::io::Error>> where M: Movetext<Output = O> {
        let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);
        
        Self::from_reader::<Cursor<&str>, M>(&mut reader)
    }
    
    /// Reads all games in this reader.
    ///
    /// # Errors
    /// See [`pgn_reader::BufferedReader::read_game`].
    pub fn from_reader<R, M>(reader: &mut BufferedReader<R>) -> Vec<Result<Self, std::io::Error>> where R: Read, M: Movetext<Output = O> {
        let mut pgns = Vec::new();

        loop {
            let mut pgn = Self::default();
            let mut pgn_visitor = Visitor::<M>::new(&mut pgn);

            let result = reader.read_game(&mut pgn_visitor);
            pgn_visitor.end_game();

            match result {
                Ok(Some(())) => pgns.push(Ok(pgn)),
                Err(e) => pgns.push(Err(e)),
                // Empty reader
                Ok(None) => break,
            }
        }

        pgns
    }
}

impl<O> Display for Pgn<O> where O: Display {
    /// Returns the string representation of this PGN.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! push_pgn_header {
            ($field:ident, $header:expr) => {
                if let Some($field) = &self.$field {
                    paste::paste! {
                        f.write_str(&::fast_concat::fast_concat!("[", $header, " \"", &$field.decode_utf8_lossy(), "\"]\n"))?;
                    }
                }
            };

            (custom_type: $field:ident, $header:expr) => {
                if let Some(Ok($field)) = &self.$field {
                    paste::paste! {
                        f.write_str(&::fast_concat::fast_concat!("[", $header, " \"", &$field.to_string(), "\"]\n"))?;
                    }
                }
            };
        }

        macro_rules! any_fields_some {
            ($($field:ident),+) => {
                $(self.$field.is_some())||+
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
        push_pgn_header!(custom_type: fen, "FEN");

        let Some(movetext) = &self.movetext else {
            return Ok(());
        };

        if any_fields_some!(event, site, date, round, white, black, outcome, white_elo, black_elo, eco, time_control, fen) {
            f.write_char('\n')?;
        }

        movetext.fmt(f)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::unreachable)]
#[allow(clippy::panic)]
mod tests {
    use test_case::test_case;
    use crate::samples::*;
    use crate::{SimpleMovetext, VariationMovetext, VariationMovetextImpl};

    #[test_case(&simple0())]
    #[test_case(&simple1())]
    fn simple_to_pgn_from_pgn(sample: &PgnSample<SimpleMovetext>) {
        sample.test::<SimpleMovetext>();
    }

    #[test_case(&variation0())]
    #[test_case(&variation1())]
    #[test_case(&variation2())]
    fn variation_to_pgn_from_pgn(sample: &PgnSample<VariationMovetext>) {
        sample.test::<VariationMovetextImpl>();
    }
}