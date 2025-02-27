use std::fmt::{Display, Formatter};
use std::str::FromStr;
use deranged::RangedU8;
use crate::EcoCategory;

/// The ECO (Encyclopaedia of Chess Openings) code of an opening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Eco {
    pub category: EcoCategory,
    pub subcategory: RangedU8<0, 99>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    NotAscii,
    NoCategory,
    NoSubcategoryFirstDigit,
    NoSubcategorySecondDigit,
    InvalidSubcategoryFirstDigit,
    InvalidSubcategorySecondDigit,
    /// Refer to [`EcoCategory::try_from`].
    InvalidCategory,
}

impl Display for Eco {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:0>2}", <EcoCategory as Into<char>>::into(self.category), self.subcategory)
    }
}

impl FromStr for Eco {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(Self::Err::NotAscii);
        }

        let mut chars = s.chars();
        let Some(first) = chars.next() else {
            return Err(Self::Err::NoCategory);
        };
        let Ok(category) = EcoCategory::try_from(first) else {
            return Err(Self::Err::InvalidCategory)
        };
        let Some(second) = chars.next() else {
            return Err(Self::Err::NoSubcategoryFirstDigit);
        };
        let Some(Ok(second)) = second.to_digit(10).map(u8::try_from) else {
            return Err(Self::Err::InvalidSubcategoryFirstDigit);
        };
        let Some(third) = chars.next() else {
            return Err(Self::Err::NoSubcategorySecondDigit);
        };
        let Some(Ok(third)) = third.to_digit(10).map(u8::try_from) else {
            return Err(Self::Err::InvalidSubcategorySecondDigit);
        };

        // SAFETY: Both of these numbers are 0-9. They can't be larger than 99 in this calculation.
        #[allow(unsafe_code)]
        #[allow(clippy::arithmetic_side_effects)]
        Ok(Self { category, subcategory: unsafe { RangedU8::new_unchecked(second * 10 + third) } })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use proptest::proptest;
    use test_case::test_case;

    macro_rules! ru8 {
        ($lit:literal) => {
            RangedU8::new_static::<$lit>()
        };
    }

    #[test_case(Eco { category: EcoCategory::A, subcategory: ru8!(9) }, "A09")]
    #[test_case(Eco { category: EcoCategory::B, subcategory: ru8!(99) }, "B99")]
    #[test_case(Eco { category: EcoCategory::C, subcategory: ru8!(9) }, "C09")]
    #[test_case(Eco { category: EcoCategory::D, subcategory: ru8!(10) }, "D10")]
    #[test_case(Eco { category: EcoCategory::E, subcategory: ru8!(99) }, "E99")]
    #[test_case(Eco { category: EcoCategory::A, subcategory: ru8!(6) }, "A06")]
    #[test_case(Eco { category: EcoCategory::B, subcategory: ru8!(12) }, "B12")]
    #[test_case(Eco { category: EcoCategory::C, subcategory: ru8!(0) }, "C00")]
    fn to_string_from_string(eco: Eco, eco_str: &str) {
        assert_eq!(eco.to_string(), eco_str);
        assert_eq!(Eco::from_str(eco_str).unwrap(), eco);
    }

    proptest! {
        #[test]
        fn from_valid_string(category in "[a-eA-E]", subcategory: u8) {
            Eco::from_str(&format!("{category}{subcategory:0>2}")).unwrap();
        }

        #[test]
        fn from_invalid_category(category in "[^a-eA-E]", subcategory: u8) {
            assert!(Eco::from_str(&format!("{category}{subcategory:0>2}")).is_err());
        }
    }
}