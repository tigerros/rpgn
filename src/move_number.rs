use std::num::NonZeroUsize;
use shakmaty::Color;

/// A struct with methods relevant to the number used in the PGN notation, backed by a `usize`.
/// 
/// Think of the backing field of the [`MoveNumber`] as an "index",
/// becasue it starts at 0 as opposed to the move numbers in the PGN notation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveNumber(pub usize);

impl MoveNumber {
    pub const MIN: Self = Self(0);
    
    pub const fn from_color_and_number(color: Color, number: NonZeroUsize) -> Self {
        // CLIPPY: Since `number` is a non-zero number, `number.get() - 1` will never overflow.
        // And there's never going to be enough moves for * 2 to cause an overflow.
        #[allow(clippy::arithmetic_side_effects)]
        match color {
            Color::White => Self((number.get() - 1) * 2),
            Color::Black => Self(((number.get() - 1) * 2) + 1)
        }
    }

    /// What side has played a move to get to this move number.
    pub const fn color(self) -> Color {
        if self.0 % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    /// This is the "number" of the move, aka. what is shown in a PGN.
    /// For example, white's first move has an "index" of 0, but a number of 1.
    /// Black's first move has an index of 1, and a number of 1 also.
    #[allow(clippy::missing_panics_doc)] // CLIPPY: Doesn't actually panic
    pub const fn number(self) -> NonZeroUsize {
        // CLIPPY: usize / 2 >= 0 so usize::MAX > (1 + usize / 2) > 0
        #[allow(clippy::arithmetic_side_effects)]
        #[allow(clippy::unwrap_used)]
        NonZeroUsize::new(1 + (self.0 / 2)).unwrap()
    }

    /// Returns how many moves white has played before this move number was reached.
    /// E.g. for `MoveNumber { index: 0 }` this is 0, for `MoveNumber { index: 1 }` it is 1.
    pub const fn white_move_count(self) -> usize {
        #[allow(clippy::arithmetic_side_effects)]
        {
            (self.0.saturating_add(1)) / 2
        }
    }

    /// Returns how many moves black has played before this move number was reached.
    /// E.g. for `MoveNumber { index: 0 }` and `MoveNumber { index: 1 }` this is 0, for `MoveNumber { index: 2 }` it is 1.
    pub const fn black_move_count(self) -> usize {
        self.0 / 2
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(1).unwrap()), 0)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(1).unwrap()), 1)]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(2).unwrap()), 2)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(2).unwrap()), 3)]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(3).unwrap()), 4)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(3).unwrap()), 5)]
    fn from_color_and_number(move_number: MoveNumber, correct_index: usize) {
        assert_eq!(move_number.0, correct_index);
    }

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(1).unwrap()), Color::White)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(1).unwrap()), Color::Black)]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(2).unwrap()), Color::White)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(2).unwrap()), Color::Black)]
    fn color(move_number: MoveNumber, correct_color: Color) {
        assert_eq!(move_number.color(), correct_color);
    }

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(1).unwrap()), NonZeroUsize::new(1).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(1).unwrap()), NonZeroUsize::new(1).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(2).unwrap()), NonZeroUsize::new(2).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(2).unwrap()), NonZeroUsize::new(2).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(3).unwrap()), NonZeroUsize::new(3).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(3).unwrap()), NonZeroUsize::new(3).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(4).unwrap()), NonZeroUsize::new(4).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(4).unwrap()), NonZeroUsize::new(4).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(5).unwrap()), NonZeroUsize::new(5).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(5).unwrap()), NonZeroUsize::new(5).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroUsize::new(6).unwrap()), NonZeroUsize::new(6).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroUsize::new(6).unwrap()), NonZeroUsize::new(6).unwrap())]
    fn number(move_number: MoveNumber, correct_number: NonZeroUsize) {
        assert_eq!(move_number.number(), correct_number);
    }

    #[test_case(MoveNumber(0), 0)]
    #[test_case(MoveNumber(1), 1)]
    #[test_case(MoveNumber(2), 1)]
    #[test_case(MoveNumber(3), 2)]
    #[test_case(MoveNumber(4), 2)]
    #[test_case(MoveNumber(5), 3)]
    fn white_move_count(move_number: MoveNumber, correct_white_move_count: usize) {
        assert_eq!(move_number.white_move_count(), correct_white_move_count);
    }

    #[test_case(MoveNumber(0), 0)]
    #[test_case(MoveNumber(1), 0)]
    #[test_case(MoveNumber(2), 1)]
    #[test_case(MoveNumber(3), 1)]
    #[test_case(MoveNumber(4), 2)]
    #[test_case(MoveNumber(5), 2)]
    fn black_move_count(move_number: MoveNumber, correct_black_move_count: usize) {
        assert_eq!(move_number.black_move_count(), correct_black_move_count);
    }
}