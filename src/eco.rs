use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::EcoCategory;

/// The ECO (Encyclopaedia of Chess Openings) code of an opening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Eco {
    pub category: EcoCategory,
    subcategory: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    NotAscii,
    NoCategory,
    NoSubcategory,
    /// Refer to [`EcoCategory::try_from`].
    InvalidCategory,
    InvalidSubcategory,
    SubcategoryGreaterThan99,
}

impl Eco {
    /// # Errors
    /// 
    /// Only possible variant is [`Error::SubcategoryGreaterThan99`].
    pub const fn new(category: EcoCategory, subcategory: u8) -> Result<Self, Error> {
        if subcategory > 99 {
            Err(Error::SubcategoryGreaterThan99)
        } else {
            Ok(Self {
                category,
                subcategory
            })
        }
    }

    /// Guaranteed to be less than 100.
    pub const fn get_subcategory(self) -> u8 {
        self.subcategory
    }

    /// # Errors
    /// 
    /// Only possible variant is [`Error::SubcategoryGreaterThan99`].
    pub fn set_subcategory(&mut self, new_subcategory: u8) -> Result<(), Error> {
        if new_subcategory > 99 {
            Err(Error::SubcategoryGreaterThan99)
        } else {
            self.subcategory = new_subcategory;
            Ok(())
        }
    }
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
            return Err(Self::Err::NoSubcategory);
        };
        let Some(third) = chars.next() else {
            let Some(Ok(second)) = second.to_digit(10).map(u32::try_into) else {
                return Err(Self::Err::InvalidSubcategory);
            };
            
            return Self::new(category, second).map_err(|_| Self::Err::SubcategoryGreaterThan99);
        };
        let mut combined = String::with_capacity(2);
        combined.push(second);
        combined.push(third);

        let Ok(subcategory) = u8::from_str(&combined) else {
            return Err(Self::Err::InvalidSubcategory);
        };

        Self::new(category, subcategory).map_err(|_| Self::Err::SubcategoryGreaterThan99)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use proptest::proptest;
    use test_case::test_case;

    #[test_case(Eco { category: EcoCategory::A, subcategory: 9 }, "A09")]
    #[test_case(Eco { category: EcoCategory::B, subcategory: 99 }, "B99")]
    #[test_case(Eco { category: EcoCategory::C, subcategory: 9 }, "C09")]
    #[test_case(Eco { category: EcoCategory::D, subcategory: 10 }, "D10")]
    #[test_case(Eco { category: EcoCategory::E, subcategory: 99 }, "E99")]
    #[test_case(Eco { category: EcoCategory::A, subcategory: 6 }, "A06")]
    #[test_case(Eco { category: EcoCategory::B, subcategory: 12 }, "B12")]
    #[test_case(Eco { category: EcoCategory::C, subcategory: 0 }, "C00")]
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

        #[test]
        fn from_invalid_subcategory(category in "[a-eA-E]", subcategory in 100..255) {
            // CLIPPY: The proptest range of subcategory makes sure none of these warnings matter.
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::as_conversions)]
            {
                assert!(Eco::new(EcoCategory::try_from(char::from_str(&category).unwrap()).unwrap(), subcategory as u8).is_err());
            }
        }
    }
}