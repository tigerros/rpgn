use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Round {
    Normal(u32),
    Multipart(Vec<u32>),
    Unknown
}

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

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            let split = s.split('.').map_while(|char| char.parse::<u32>().ok());

            return Ok(Self::Multipart(split.collect()));
        }
        
        s.parse().map_or_else(|_| if s == "?" {
            Ok(Self::Unknown)
        } else {
            Err(())
        }, |parsed| Ok(Self::Normal(parsed)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use test_case::test_case;
    
    #[test_case(&Round::Normal(79), "79")]
    #[test_case(&Round::Normal(4_294_967_295), "4294967295")]
    #[test_case(&Round::Normal(0), "0")]
    #[test_case(&Round::Multipart(vec![3, 7, 1]), "3.7.1")]
    #[test_case(&Round::Multipart(vec![200, 1000, 0, 1]), "200.1000.0.1")]
    #[test_case(&Round::Unknown, "?")]
    fn to_string_from_string(round: &Round, round_str: &str) {
        assert_eq!(round.to_string(), round_str);
        assert_eq!(&Round::from_str(round_str).unwrap(), round);
    }
}