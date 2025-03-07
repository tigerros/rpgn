use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::io::Read;
use std::str::FromStr;
use fast_concat::fast_concat;
use pgn_reader::BufferedReader;
use shakmaty::fen::{Fen, ParseFenError};
use super::visitor::{Visitor};
use crate::{Eco, Outcome, Date, Round, RawHeaderOwned, Movetext};

/// The generic `M` should be a struct that implements [`Movetext`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pgn<M> {
    /// See "Event" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN158>
    pub event: Option<RawHeaderOwned>,
    /// See "Site" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN164>
    pub site: Option<RawHeaderOwned>,
    /// See "Date" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN170>
    pub date: Option<Result<Date, <Date as FromStr>::Err>>,
    /// See "Round" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN176>
    pub round: Option<Result<Round, <Round as FromStr>::Err>>,
    /// See "White" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN183>
    pub white: Option<RawHeaderOwned>,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.1.2>
    pub white_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// See "Black" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN191>
    pub black: Option<RawHeaderOwned>,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.1.2>
    pub black_elo: Option<Result<u16, <u16 as FromStr>::Err>>,
    /// See "Result" under "Seven Tag Roster".
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#AEN197>
    pub outcome: Option<Result<Outcome, <Outcome as FromStr>::Err>>,
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.4.1>
    pub eco: Option<Result<Eco, <Eco as FromStr>::Err>>,
    // TODO: Make a time control type
    /// Not typed yet.
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.6>
    pub time_control: Option<RawHeaderOwned>,
    /// Note that this FEN may not be a legal position.
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c9.7.2>
    pub fen: Option<Result<Fen, ParseFenError>>,
    /// Other headers which I haven't implemented yet. Doesn't allocate if there's no other headers.
    ///
    /// The headers are processed sequentially, so if there's identical headers,
    /// the value of the last one wins.
    pub other_headers: HashMap<Vec<u8>, RawHeaderOwned>,
    /// The actual game. See [`Movetext`].
    /// <https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c8.2>
    pub movetext: M,
}

impl<M> Default for Pgn<M> where M: Movetext {
    /// Creates a [`Pgn`] with all fields set to [`None`], and calls [`Default::default`] on `M`.
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
            other_headers: HashMap::new(),
            movetext: M::default(),
        }
    }
}

impl<M> Pgn<M> where M: Movetext {
    #[allow(clippy::should_implement_trait)]
    /// Reads all games in this string.
    ///
    /// # Errors
    /// See [`pgn_reader::BufferedReader::read_game`].
    pub fn from_str(pgn: &str) -> Vec<Result<Self, std::io::Error>> {
        let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);
        
        Self::from_reader(&mut reader)
    }
    
    /// Reads all games in this reader.
    ///
    /// # Errors
    /// See [`pgn_reader::BufferedReader::read_game`].
    pub fn from_reader<R>(reader: &mut BufferedReader<R>) -> Vec<Result<Self, std::io::Error>> where R: Read {
        let mut pgns = Vec::new();

        loop {
            let mut pgn = Self::default();
            let mut pgn_visitor = Visitor::new(&mut pgn);

            let result = reader.read_game(&mut pgn_visitor);

            match result {
                Ok(Some(())) => {
                    pgn_visitor.end_game();
                    pgns.push(Ok(pgn));
                },
                Err(e) => pgns.push(Err(e)),
                // Empty reader
                Ok(None) => break,
            }
        }

        pgns
    }
}

impl<M> Display for Pgn<M> where M: Display {
    /// Returns the string representation of this PGN.
    ///
    /// Types such as `Vec<u8>` are lossily decoded as UTF-8.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        macro_rules! push_pgn_header {
            ($field:ident, $header:expr) => {
                if let Some($field) = &self.$field {
                    f.write_str(&fast_concat!("[", $header, " \"", &$field.decode_utf8_lossy(), "\"]\n"))?;
                }
            };

            (custom_type: $field:ident, $header:expr) => {
                if let Some(Ok($field)) = &self.$field {
                    f.write_str(&fast_concat!("[", $header, " \"", &$field.to_string(), "\"]\n"))?;
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

        for (key, value) in &self.other_headers {
            f.write_str(&fast_concat!("[", &String::from_utf8_lossy(key), " \"", &value.decode_utf8_lossy(), "\"]\n"))?;
        }

        if any_fields_some!(event, site, date, round, white, black, outcome, white_elo, black_elo, eco, time_control, fen) {
            f.write_char('\n')?;
        }

        self.movetext.fmt(f)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::unreachable)]
#[allow(clippy::panic)]
mod tests {
    use shakmaty::san::SanPlus;
    use test_case::test_case;
    use crate::movetext::{Sans, Variation};
    use crate::samples::*;

    #[test_case(&sans0())]
    #[test_case(&sans1())]
    fn san_vec_to_pgn_from_pgn(sample: &PgnSample<Sans<SanPlus>>) {
        sample.test();
    }

    #[test_case(&variation0())]
    #[test_case(&variation1())]
    #[test_case(&variation2())]
    fn variation_to_pgn_from_pgn(sample: &PgnSample<Variation<SanPlus>>) {
        sample.test();
    }
}