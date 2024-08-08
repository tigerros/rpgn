use std::io::Read;
use pgn_reader::BufferedReader;
use shakmaty::san::{San, SanPlus, Suffix};
use super::visitor::{Visitor, VisitorSanError};
use crate::{Eco, game::{Outcome, Date, Round}, MoveNumber, Variation, Turn};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
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

impl Game {
    #[allow(clippy::should_implement_trait)]
    /// Reads all games in this string.
    ///
    /// # Errors
    ///
    /// These are errors for every item in the `Vec`. This function does not error itself.
    ///
    /// - [`PgnParseError::Io`]: an IO error occurred.
    /// - [`PgnParseError::EmptyReader`]: the string is empty.
    /// - [`PgnParseError::SanError`]: there is an illegal SAN in the PGN.
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
    ///
    /// - [`PgnParseError::Io`]: an IO error occurred.
    /// - [`PgnParseError::EmptyReader`]: the string is empty.
    /// - [`PgnParseError::SanError`]: there is an illegal SAN in the PGN.
    pub fn from_reader<R>(reader: &mut BufferedReader<R>) -> Vec<Result<Self, PgnParseError>> where R: Read {
        let mut games = Vec::new();

        loop {
            let mut game_visitor = Visitor::new();

            let result = reader.read_game(&mut game_visitor);

            match result {
                Ok(Some(())) => match game_visitor.into_game() {
                    Ok(game) => games.push(Ok(game)),
                    Err(e) => games.push(Err(PgnParseError::SanError(e))),
                },
                Err(e) => games.push(Err(PgnParseError::Io(e))),
                // Empty reader
                Ok(None) => break,
            }
        }

        games
    }

    pub fn to_pgn(&self) -> String {
        let mut pgn = String::with_capacity(300);

        macro_rules! push_pgn_header {
            ($field_name:ident) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        pgn.push_str(&crate::concat_strings!("[", stringify!([<$field_name:camel>]), " \"", $field_name, "\"]\n"));
                    }
                }
            };

            ($field_name:ident, $header_title:expr) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        pgn.push_str(&crate::concat_strings!("[", $header_title, " \"", $field_name, "\"]\n"));
                    }
                }
            };

            (non_str_display: $field_name:ident) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        pgn.push_str(&crate::concat_strings!("[", stringify!([<$field_name:camel>]), " \"", &$field_name.to_string(), "\"]\n"));
                    }
                }
            };

            (non_str_display: $field_name:ident, $header_title:expr) => {
                if let Some($field_name) = &self.$field_name {
                    paste::paste! {
                        pgn.push_str(&crate::concat_strings!("[", $header_title, " \"", &$field_name.to_string(), "\"]\n"));
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
            return pgn;
        };

        pgn.push('\n');
        pgn.push_str(&root_variation.to_string());
        pgn
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
mod tests {
    use shakmaty::{Chess, Color, Move, Position, san::SanError};
    use crate::{EcoCategory, TurnsCapacity, variation::play_moves, VariationsCapacity};
    use super::*;
    use test_case::test_case;
    use pretty_assertions::{assert_eq};
    use std::num::{NonZeroU16, NonZeroU8};
    use std::str::FromStr;
    use std::time::Instant;
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

    fn pgn1_parsed() -> Game {
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

        Game {
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

    // fn pgn2_parsed() -> Game {
    //     let mut root_var = Variation::new(Chess::new());
    // 
    //     play_san_strings!(
    //         root_var,
    //         "g4",
    //         "e5",
    //         "f3",
    //         "Qh4"
    //     ).unwrap();
    // 
    //     Game {
    //         event: Some("Live Chess".to_string()),
    //         site: Some("Lichess".to_string()),
    //         date: Some(Date::new(Some(2024), Some(unsafe { NonZeroU8::new_unchecked(2) }), None).unwrap()),
    //         white: Some("Nasrin_Babayeva".to_string()),
    //         white_elo: Some(1765),
    //         black: Some("tigerros0".to_string()),
    //         black_elo: Some(1584),
    //         outcome: Some(Outcome::Decisive { winner: Color::Black }),
    //         round: Some(Round::Multipart(vec![3, 1, 2])),
    //         eco: Some(Eco::new(EcoCategory::A, 00).unwrap()),
    //         time_control: Some("600+2".to_string()),
    //         root_variation: Some(root_var),
    //     }
    // }
    //
    // fn pgn3_parsed() -> Game {
    //     let mut correct_root_variation = Variation::new_starting_root_variation();
    //
    //     correct_root_variation.push_move(&San::from_ascii(b"e4").unwrap()).unwrap();
    //
    //     let mut d4_variation = correct_root_variation.new_variation_at_last_move(2);
    //
    //     play_moves!(
    //         d4_variation,
    //         &San::from_ascii(b"d4").unwrap(),
    //         &San::from_ascii(b"d5").unwrap()
    //     ).unwrap();
    //
    //     let mut f5_variation = d4_variation.new_variation_at_last_move(3);
    //
    //     play_moves!(
    //         f5_variation,
    //         &San::from_ascii(b"f5").unwrap(),
    //         &San::from_ascii(b"g3").unwrap(),
    //         &San::from_ascii(b"Nf6").unwrap()
    //     ).unwrap();
    //
    //     let mut c4_variation = f5_variation.new_variation_at(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(2).unwrap()), 5).unwrap();
    //
    //     play_moves!(
    //         c4_variation,
    //         &San::from_ascii(b"c4").unwrap(),
    //         &San::from_ascii(b"Nf6").unwrap(),
    //         &San::from_ascii(b"Nc3").unwrap(),
    //         &San::from_ascii(b"e6").unwrap(),
    //         &San::from_ascii(b"Nf3").unwrap()
    //     ).unwrap();
    //
    //     let mut g6_variation = c4_variation.new_variation_at(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(3).unwrap()), 1).unwrap();
    //
    //     g6_variation.push_move(&San::from_ascii(b"g6").unwrap()).unwrap();
    //
    //     play_moves!(
    //         correct_root_variation,
    //         &San::from_ascii(b"e5").unwrap(),
    //         &San::from_ascii(b"Nf3").unwrap(),
    //         &San::from_ascii(b"Nc6").unwrap(),
    //         &San::from_ascii(b"Bc4").unwrap(),
    //         &San::from_ascii(b"Nf6").unwrap()
    //     ).unwrap();
    //
    //     let mut bc5_variation = correct_root_variation.new_variation_at_last_move(1);
    //
    //     bc5_variation.push_move(&San::from_ascii(b"Bc5").unwrap()).unwrap();
    //     correct_root_variation.push_move(&San::from_ascii(b"d3").unwrap()).unwrap();
    //
    //     c4_variation.insert_variation(g6_variation);
    //     f5_variation.insert_variation(c4_variation);
    //     d4_variation.insert_variation(f5_variation);
    //     correct_root_variation.insert_variation(d4_variation);
    //     correct_root_variation.insert_variation(bc5_variation);
    //
    //     Game {
    //         event: None,
    //         site: None,
    //         date: Some(Date::new(None, Some(unsafe { NonZeroU8::new_unchecked(1) }), None).unwrap()),
    //         white: None,
    //         black: None,
    //         outcome: Some(Outcome::Draw),
    //         round: Some(Round::Normal(1)),
    //         white_elo: None,
    //         black_elo: None,
    //         eco: Some(Eco::new(EcoCategory::C, 50).unwrap()),
    //         time_control: None,
    //         root_variation: Some(correct_root_variation),
    //     }
    // }

    #[test_case(PGN1, &pgn1_parsed())]
    //#[test_case(PGN2, Ok(pgn2_parsed()))]
    //#[test_case(PGN3, Ok(pgn3_parsed()))]
    fn to_pgn_from_pgn(pgn: &str, correct_game: &Game) {
        let start = Instant::now();
        let _ = correct_game.to_pgn();
        let elapsed = start.elapsed();
        println!("To PGN elapsed: {elapsed:?}");
        
        let start = Instant::now();
        let _ = Game::from_str(pgn);
        let elapsed = start.elapsed();
        println!("From string PGN elapsed: {elapsed:?}");
        assert_eq!(correct_game.to_pgn(), pgn);
        assert_eq!(&Game::from_str(pgn).first().unwrap().as_ref().unwrap(), &correct_game);
    }

    #[test]
    fn from_reader() {
        let mut reader = BufferedReader::new_cursor(PGN1.to_string());// + "\n" + PGN2 + "\n" + PGN3);
        
        let start = Instant::now();
        let result = Game::from_reader(&mut reader);
        let elapsed = start.elapsed();
        println!("From reader PGN elapsed: {elapsed:?}");
        assert_eq!(
            &result.into_iter().filter_map(Result::ok).collect::<Vec<_>>(),
            &[pgn1_parsed(), ]//pgn2_parsed(), pgn3_parsed()]
        );
    }
}