use std::num::NonZeroU16;
use shakmaty::Color;

/// A wrapper around a `u16` with some methods that are relevant to a move number.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveNumber {
    /// The backing field of the [`MoveNumber`].
    /// It's called an index because it starts at 0 as opposed to the move numbers in the PGN notation.
    pub index: u16,
}

impl MoveNumber {
    pub const MIN: Self = Self { index: 0 };
    pub const fn from_color_and_number(color: Color, number: NonZeroU16) -> Self {
        // CLIPPY: Since `number` is a non-zero number, `number.get() - 1` will never overflow.
        // And there's never going to be enough moves for * 2 to cause an overflow.
        #[allow(clippy::arithmetic_side_effects)]
        match color {
            Color::White => Self { index: (number.get() - 1) * 2 },
            Color::Black => Self { index: ((number.get() - 1) * 2) + 1 }
        }
    }

    pub const fn color(self) -> Color {
        if self.index % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    /// This is the "number" of the move, aka. what is shown in a PGN.
    /// For example, white's first move has an "index" of 0, but a number of 1.
    /// Black's first move has an index of 1, and a number of 1 also.
    pub const fn number(self) -> NonZeroU16 {
        // SAFETY & CLIPPY: u16 / 2 >= 0 so u16::MAX > (1 + u16 / 2) > 0
        #[allow(clippy::arithmetic_side_effects)]
        unsafe { NonZeroU16::new_unchecked(1 + (self.index / 2)) }
    }

    /// Returns how many moves white has played before this move number was reached.
    /// E.g. for `MoveNumber { index: 0 }` this is 0, for `MoveNumber { index: 1 }` it is 1.
    pub const fn white_move_count(self) -> u16 {
        (self.index + 1) / 2
    }

    /// Returns how many moves black has played before this move number was reached.
    /// E.g. for `MoveNumber { index: 0 }` and `MoveNumber { index: 1 }` this is 0, for `MoveNumber { index: 2 }` it is 1.
    pub const fn black_move_count(self) -> u16 {
        self.index / 2
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(1).unwrap()), 0)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(1).unwrap()), 1)]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(2).unwrap()), 2)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(2).unwrap()), 3)]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(3).unwrap()), 4)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(3).unwrap()), 5)]
    fn from_color_and_number(move_number: MoveNumber, correct_index: u16) {
        assert_eq!(move_number.index, correct_index);
    }

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(1).unwrap()), Color::White)]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(1).unwrap()), Color::Black)]
    fn color(move_number: MoveNumber, correct_color: Color) {
        assert_eq!(move_number.color(), correct_color);
    }

    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(1).unwrap()), NonZeroU16::new(1).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(1).unwrap()), NonZeroU16::new(1).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(2).unwrap()), NonZeroU16::new(2).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(2).unwrap()), NonZeroU16::new(2).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(3).unwrap()), NonZeroU16::new(3).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(3).unwrap()), NonZeroU16::new(3).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(4).unwrap()), NonZeroU16::new(4).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(4).unwrap()), NonZeroU16::new(4).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(5).unwrap()), NonZeroU16::new(5).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(5).unwrap()), NonZeroU16::new(5).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::White, NonZeroU16::new(6).unwrap()), NonZeroU16::new(6).unwrap())]
    #[test_case(MoveNumber::from_color_and_number(Color::Black, NonZeroU16::new(6).unwrap()), NonZeroU16::new(6).unwrap())]
    fn number(move_number: MoveNumber, correct_number: NonZeroU16) {
        assert_eq!(move_number.number(), correct_number);
    }

    #[test_case(MoveNumber { index: 0 }, 0)]
    #[test_case(MoveNumber { index: 1 }, 1)]
    #[test_case(MoveNumber { index: 2 }, 1)]
    #[test_case(MoveNumber { index: 3 }, 2)]
    #[test_case(MoveNumber { index: 4 }, 2)]
    #[test_case(MoveNumber { index: 5 }, 3)]
    fn white_move_count(move_number: MoveNumber, correct_white_move_count: u16) {
        assert_eq!(move_number.white_move_count(), correct_white_move_count);
    }

    #[test_case(MoveNumber { index: 0 }, 0)]
    #[test_case(MoveNumber { index: 1 }, 0)]
    #[test_case(MoveNumber { index: 2 }, 1)]
    #[test_case(MoveNumber { index: 3 }, 1)]
    #[test_case(MoveNumber { index: 4 }, 2)]
    #[test_case(MoveNumber { index: 5 }, 2)]
    fn black_move_count(move_number: MoveNumber, correct_black_move_count: u16) {
        assert_eq!(move_number.black_move_count(), correct_black_move_count);
    }
}