use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EcoCategory {
    A,
    B,
    C,
    D,
    E
}

impl TryFrom<char> for EcoCategory {
    type Error = ();

    /// The error type is blank because there's only one error scenario: the character
    /// doesn't match one of the enum variants.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            _ => Err(()),
        }
    }
}

// Clippy: From<char> is impossible to implement because it might fail, that's why I used TryFrom<char> instead.
#[allow(clippy::from_over_into)]
impl Into<char> for EcoCategory {
    fn into(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use proptest::proptest;
    use test_case::test_case;
    use std::str::FromStr;

    #[test_case(EcoCategory::A, 'A')]
    #[test_case(EcoCategory::B, 'B')]
    #[test_case(EcoCategory::C, 'C')]
    #[test_case(EcoCategory::D, 'D')]
    #[test_case(EcoCategory::E, 'E')]
    fn to_char_from_char(eco_category: EcoCategory, eco_category_char: char) {
        assert_eq!(EcoCategory::try_from(eco_category_char.to_ascii_lowercase()).unwrap(), eco_category);
        assert_eq!(EcoCategory::try_from(eco_category_char.to_ascii_uppercase()).unwrap(), eco_category);
        assert_eq!(<EcoCategory as Into<char>>::into(eco_category), eco_category_char.to_ascii_uppercase());
    }

    proptest! {
        #[test]
        fn from_invalid_char(c in "[^a-eA-E]") {
            assert!(EcoCategory::try_from(char::from_str(&c).unwrap()).is_err());
        }
    }
}