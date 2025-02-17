use crate::{Date, Eco, Movetext, Outcome, Pgn, Round};
use pgn_reader::{RawHeader, Skip};
use shakmaty::fen::Fen;
use shakmaty::san::SanPlus;
use std::str::FromStr;

/// Mutates the given [`Pgn`].
/// Remember to call [`Visitor::finalize_movetext`] **after** using the visitor.
pub struct Visitor<'pgn, M>
where
    M: Movetext,
{
    pgn: &'pgn mut Pgn<M>,
    movetext_agent: Option<M::Agent>,
}

impl<'pgn, M> Visitor<'pgn, M>
where
    M: Movetext,
{
    pub fn new(pgn: &'pgn mut Pgn<M>) -> Self {
        Visitor {
            pgn,
            movetext_agent: None,
        }
    }

    /// This sets the `Pgn.movetext` field.
    /// Call this after using the visitor.
    pub fn end_game(self) {
        self.pgn.movetext = self.movetext_agent.map(Movetext::end_game);
    }
}

impl<M> pgn_reader::Visitor for Visitor<'_, M>
where
    M: Movetext,
{
    type Result = ();

    fn header(&mut self, key: &[u8], raw_header: RawHeader<'_>) {
        match key.to_ascii_lowercase().as_slice() {
            b"event" => self.pgn.event = Some(raw_header.into()),
            b"site" => self.pgn.site = Some(raw_header.into()),
            b"date" => self.pgn.date = Some(Date::from_str(&raw_header.decode_utf8_lossy())),
            b"white" => self.pgn.white = Some(raw_header.into()),
            b"black" => self.pgn.black = Some(raw_header.into()),
            b"whiteelo" => self.pgn.white_elo = Some(raw_header.decode_utf8_lossy().parse()),
            b"blackelo" => self.pgn.black_elo = Some(raw_header.decode_utf8_lossy().parse()),
            b"result" => {
                self.pgn.outcome = Some(Outcome::from_str(&raw_header.decode_utf8_lossy()));
            }
            b"round" => self.pgn.round = Some(Round::from_str(&raw_header.decode_utf8_lossy())),
            b"eco" => self.pgn.eco = Some(Eco::from_str(&raw_header.decode_utf8_lossy())),
            b"timecontrol" => self.pgn.time_control = Some(raw_header.into()),
            b"fen" => self.pgn.fen = Some(Fen::from_str(&raw_header.decode_utf8_lossy())),
            _ => {}
        }
    }

    fn begin_game(&mut self) {
        self.movetext_agent = Some(M::begin_game());
    }

    fn begin_variation(&mut self) -> Skip {
        self.movetext_agent
            .as_mut()
            .map_or(Skip(true), M::begin_variation)
    }

    fn end_variation(&mut self) {
        if let Some(movetext_agent) = &mut self.movetext_agent {
            M::end_variation(movetext_agent);
        }
    }

    fn san(&mut self, san: SanPlus) {
        if let Some(movetext_agent) = &mut self.movetext_agent {
            M::san(movetext_agent, san);
        }
    }

    fn end_game(&mut self) {}
}
