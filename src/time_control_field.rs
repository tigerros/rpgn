use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TimeControlField {
    Unknown,
    NoTimeControl,
    MovesPerSeconds { moves: u32, seconds: u32 },
    Seconds(u32),
    Increment { seconds: u32, increment_seconds: u32 },
    Hourglass { seconds: u32 }
}

#[derive(Copy, Clone, Debug)]
pub enum TimeControlFieldParseError {
    HourglassNumberParseError,
    /// Couldn't parse the "moves" portion in a "moves/seconds" string.
    MovesPerSecondsMovesNumberParseError,
    /// Couldn't parse the "seconds" portion in a "moves/seconds" string.
    MovesPerSecondsSecondsNumberParseError,
    NoNumberAtAll,
    /// Couldn't parse the "seconds" portion in a "seconds+increment" string.
    IncrementSecondsParseError,
    /// Couldn't parse the "increment" portion in a "seconds+increment" string.
    IncrementIncrementSecondsParseError,
}

impl FromStr for TimeControlField {
    type Err = TimeControlFieldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "?" {
            Ok(Self::Unknown)
        } else if s == "-" {
            Ok(Self::NoTimeControl)
        } else if let Some((_, seconds)) = s.split_once('*') {
            let Ok(seconds) = seconds.parse::<u32>() else {
                return Err(Self::Err::HourglassNumberParseError);
            };

            Ok(Self::Hourglass { seconds })
        } else if let Some((moves, seconds)) = s.split_once('/') {
            let Ok(moves) = moves.parse::<u32>() else {
                return Err(Self::Err::MovesPerSecondsMovesNumberParseError);
            };
            let Ok(seconds) = seconds.parse::<u32>() else {
                return Err(Self::Err::MovesPerSecondsSecondsNumberParseError);
            };

            Ok(Self::MovesPerSeconds { moves, seconds })
        } else if let Some((seconds, increment_seconds)) = s.split_once('+') {
            let Ok(seconds) = seconds.parse::<u32>() else {
                return Err(Self::Err::IncrementSecondsParseError);
            };
            let Ok(increment_seconds) = increment_seconds.parse::<u32>() else {
                return Err(Self::Err::IncrementIncrementSecondsParseError);
            };

            Ok(Self::Increment { seconds, increment_seconds })
        } else if let Ok(seconds) = s.parse::<u32>() {
            Ok(Self::Seconds(seconds))
        } else {
            Err(Self::Err::NoNumberAtAll)
        }
    }
}

impl Display for TimeControlField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_char('?'),
            Self::NoTimeControl => f.write_char('-'),
            Self::MovesPerSeconds { moves, seconds } => {
                f.write_str(&moves.to_string())?;
                f.write_char('/')?;
                f.write_str(&seconds.to_string())
            },
            Self::Seconds(seconds) => f.write_str(&seconds.to_string()),
            Self::Increment { seconds, increment_seconds } => {
                f.write_str(&seconds.to_string())?;
                f.write_char('+')?;
                f.write_str(&increment_seconds.to_string())
            },
            Self::Hourglass { seconds } => {
                f.write_char('*')?;
                f.write_str(&seconds.to_string())
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use proptest::proptest;
    use test_case::test_case;

    #[test_case(TimeControlField::Unknown, "?")]
    #[test_case(TimeControlField::NoTimeControl, "-")]
    #[test_case(TimeControlField::MovesPerSeconds { moves: 40, seconds: 9000 }, "40/9000")]
    #[test_case(TimeControlField::Seconds(600), "600")]
    #[test_case(TimeControlField::Increment { seconds: 600, increment_seconds: 10 }, "600+10")]
    #[test_case(TimeControlField::Hourglass { seconds: 1000 }, "*1000")]
    fn to_string_from_string(time_control: TimeControlField, time_control_str: &str) {
        assert_eq!(time_control.to_string(), time_control_str);
        assert_eq!(TimeControlField::from_str(time_control_str).unwrap(), time_control);
    }

    proptest! {
        #[test]
        fn moves_per_seconds(moves: u32, seconds: u32) {
            TimeControlField::from_str(&format!("{moves}/{seconds}")).unwrap();
        }
        
        #[test]
        fn increment(seconds: u32, increment_seconds: u32) {
            TimeControlField::from_str(&format!("{seconds}/{increment_seconds}")).unwrap();
        }
        
        #[test]
        fn hourglass(seconds: u32) {
            TimeControlField::from_str(&format!("*{seconds}")).unwrap();
        }
    }
}