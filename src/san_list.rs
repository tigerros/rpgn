use std::fmt::{Display, Formatter, Write};
use shakmaty::Move;
use shakmaty::san::{San, SanPlus, Suffix};
use crate::MoveNumber;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SanList(pub Vec<SanPlus>);

impl Display for SanList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut move_number = MoveNumber(0);
        let mut first_move = true;
        
        for san in &self.0 {
            if first_move {
                first_move = false;
            } else {
                f.write_char(' ')?;
            }

            move_number.number().fmt(f)?;

            if move_number.color().is_white() {
                f.write_str(". ")?;
            } else {
                f.write_str("... ")?;
            }

            san.fmt(f)?;
            
            move_number.0 += 1;
        }
        
        Ok(())
    }
}