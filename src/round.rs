use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Round {
    /// Why is it not a smaller type instead of a `usize`? I mean, humans probably won't play
    /// more than 256 rounds, but computers will.
    Normal(usize),
    Multipart(Vec<usize>),
    /// This is a successful value. "?" translates to this.
    Unknown
}

#[cfg(feature = "serde")]
crate::serde_display_from_str!(Round);

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal(round) => f.write_str(&round.to_string()),
            Self::Multipart(round_numbers) => {
                for i in 0..round_numbers.len() {
                    let Some(round_number) = round_numbers.get(i) else {
                        break;
                    };

                    f.write_str(&round_number.to_string())?;

                    // CLIPPY: if this for loop executes then round_numbers.len() must be 1 or above
                    #[allow(clippy::arithmetic_side_effects)]
                    if i < round_numbers.len() - 1 {
                        f.write_char('.')?;
                    }
                }
                
                Ok(())
            },
            Self::Unknown => f.write_char('?'),
        }
    }
}

/// Note that a string may have multiple errors, but only one error is returned.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParseError {
    /// When it contains a dot, but instead of containing only numbers separated by dots,
    /// it contains other stuff.
    InvalidMultipart {
        /// The index of the first invalid value.
        index: usize
    },
    /// There's a dot at the start of the string instead of in the middle.
    BeginningDot,
    /// There's a dot at the end of the string instead of in the middle.
    EndingDot,
    /// When there's no dot and the string is not a number or a question mark.
    InvalidSinglepart
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMultipart { index } => write!(f, "invalid multipart PGN round. Error at index: {index}"),
            Self::BeginningDot => f.write_str("dot at the start of a PGN round"),
            Self::EndingDot => f.write_str("dot at the end of a PGN round"),
            Self::InvalidSinglepart => f.write_str("invalid singlepart PGN round, i.e. failed to parse the string as a number or a question mark"),
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            if s.starts_with('.') {
                return Err(ParseError::BeginningDot);
            } else if s.ends_with('.') {
                return Err(ParseError::EndingDot);
            }

            let mut index = 0;
            let split = s.split('.');
            let mut numbers = Vec::new();

            for word in split {
                let number = word.parse().map_err(|_| ParseError::InvalidMultipart { index })?;

                numbers.push(number);
                index = index.saturating_add(word.len()).saturating_add(1);
            }

            return Ok(Self::Multipart(numbers));
        }
        
        s.parse().map_or_else(|_| if s == "?" {
            Ok(Self::Unknown)
        } else {
            Err(ParseError::InvalidSinglepart)
        }, |parsed| Ok(Self::Normal(parsed)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use test_case::test_case;
    
    #[test_case(Ok(Round::Normal(79)), "79")]
    #[test_case(Ok(Round::Normal(4_294_967_295)), "4294967295")]
    #[test_case(Ok(Round::Normal(0)), "0")]
    #[test_case(Ok(Round::Multipart(vec![3, 7, 1])), "3.7.1")]
    #[test_case(Ok(Round::Multipart(vec![200, 1000, 0, 1])), "200.1000.0.1")]
    #[test_case(Ok(Round::Unknown), "?")]
    #[test_case(Err(ParseError::InvalidSinglepart), "F")]
    #[test_case(Err(ParseError::BeginningDot), ".6.8")]
    #[test_case(Err(ParseError::BeginningDot), ".")]
    #[test_case(Err(ParseError::EndingDot), "0.7.")]
    #[test_case(Err(ParseError::InvalidMultipart { index: 2 }), "3.a.0")]
    #[test_case(Err(ParseError::InvalidMultipart { index: 2 }), "3..0")]
    #[test_case(Err(ParseError::InvalidMultipart { index: 0 }), "a.1000.0.1")]
    #[test_case(Err(ParseError::InvalidMultipart { index: 6 }), "1.100.55503480958345093458435839058439058309548594358435039589")] // I don't support these numbers :P
    fn to_string_from_string(round: Result<Round, ParseError>, round_str: &str) {
        assert_eq!(Round::from_str(round_str), round);

        if let Ok(round) = round {
            assert_eq!(round.to_string(), round_str);
        }
    }
}