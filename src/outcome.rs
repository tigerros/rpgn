use std::fmt::{Display, Formatter};
use std::str::FromStr;
use shakmaty::Color;

/// This is like [`shakmaty::Outcome`], but with an additional variant: [`Outcome::Other`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Outcome {
    Decisive {
        #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
        winner: Color
    },
    Draw,
    /// In progress, game abandoned, result otherwise unknown, or an invalid value.
    Other,
}

#[cfg(feature = "serde")]
mod color_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use shakmaty::Color;

    // CLIPPY: Serde requires a reference.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_bool(*color == Color::Black)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error> where D: Deserializer<'de> {
        let bool = bool::deserialize(deserializer)?;

        Ok(if bool {
            Color::Black
        } else {
            Color::White
        })
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Draw => "1/2-1/2",
            Self::Other => "*",
            Self::Decisive { winner: Color::White } => "1-0",
            Self::Decisive { winner: Color::Black } => "0-1",
        })
    }
}

impl FromStr for Outcome {
    type Err = ();

    /// There's only one error case: the string doesn't match any of these:
    /// - `1-0`
    /// - `0-1`
    /// - `1/2-1/2`
    /// - `*`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1-0" => Ok(Self::Decisive { winner: Color::White }),
            "0-1" => Ok(Self::Decisive { winner: Color::Black }),
            "1/2-1/2" => Ok(Self::Draw),
            "*" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use test_case::test_case;
    
    #[test_case(Outcome::Draw, "1/2-1/2")]
    #[test_case(Outcome::Other, "*")]
    #[test_case(Outcome::Decisive { winner: Color::White }, "1-0")]
    #[test_case(Outcome::Decisive { winner: Color::Black }, "0-1")]
    fn to_string_from_string(result: Outcome, result_str: &str) {
        assert_eq!(result.to_string(), result_str);
        assert_eq!(Outcome::from_str(result_str).unwrap(), result);
    }
}