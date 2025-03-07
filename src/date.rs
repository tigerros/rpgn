use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use deranged::{OptionRangedU16, OptionRangedU8, RangedU16, RangedU8};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Date {
    pub year: OptionRangedU16<0, 9999>,
    pub month: OptionRangedU8<1, 12>,
    pub day: OptionRangedU8<1, 31>
}

#[cfg(feature = "time")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntoTimeDateError {
    NoYear,
    NoMonth,
    NoDay,
    TimeError(time::error::ComponentRange)
}

#[cfg(feature = "time")]
impl TryFrom<Date> for time::Date {
    type Error = IntoTimeDateError;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
        let Some(year) = date.year.get() else { return Err(IntoTimeDateError::NoYear); };
        let Some(month) = date.month.get() else { return Err(IntoTimeDateError::NoMonth); };
        let Some(day) = date.day.get() else { return Err(IntoTimeDateError::NoDay); };

        // CLIPPY: We make sure the month is in the valid range.
        #[allow(clippy::unwrap_used)]
        Self::from_calendar_date(i32::from(year.get()), time::Month::try_from(month.get()).unwrap(), day.get()).map_err(IntoTimeDateError::TimeError)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(year) = self.year.get_primitive() {
            f.write_str(&format!("{year:0>4}"))?;
        } else {
            f.write_str("????")?;
        }

        f.write_char('.')?;

        if let Some(month) = self.month.get_primitive() {
            f.write_str(&format!("{month:0>2}"))?;
        } else {
            f.write_str("??")?;
        }

        f.write_char('.')?;

        if let Some(day) = self.day.get_primitive() {
            f.write_str(&format!("{day:0>2}"))
        } else {
            f.write_str("??")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    NoYear,
    NoMonth,
    NoDay,
}

impl FromStr for Date {
    type Err = ParseError;

    /// If parsing of a year/month/day fails, that value will be [`None`].
    /// If the value is completely missing though, an error will be returned.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');

        let Some(year) = split.next() else {
            return Err(Self::Err::NoYear);
        };

        let year = year.parse::<RangedU16<0, 9999>>().map_or(OptionRangedU16::None, OptionRangedU16::Some);

        let Some(month) = split.next() else {
            return Err(Self::Err::NoMonth);
        };

        let month = month.parse::<RangedU8<1, 12>>().map_or(OptionRangedU8::None, OptionRangedU8::Some);

        let Some(day) = split.next() else {
            return Err(Self::Err::NoDay);
        };

        let day = day.parse::<RangedU8<1, 31>>().map_or(OptionRangedU8::None, OptionRangedU8::Some);

        Ok(Self { year, month, day })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};
    use test_case::test_case;

    macro_rules! oru16 {
        ($n:literal) => {
            OptionRangedU16::Some(RangedU16::new_static::<$n>())
        };

        () => {
            OptionRangedU16::None
        };
    }

    macro_rules! oru8 {
        ($n:literal) => {
            OptionRangedU8::Some(RangedU8::new_static::<$n>())
        };

        () => {
            OptionRangedU8::None
        };
    }

    #[test_case(Date { year: oru16!(2024), month: oru8!(2), day: oru8!(1) }, "2024.02.01")]
    #[test_case(Date { year: oru16!(15), month: oru8!(12), day: oru8!() }, "0015.12.??")]
    #[test_case(Date { year: oru16!(100), month: oru8!(), day: oru8!(11) }, "0100.??.11")]
    #[test_case(Date { year: oru16!(), month: oru8!(1), day: oru8!(1) }, "????.01.01")]
    #[test_case(Date { year: oru16!(), month: oru8!(), day: oru8!(1) }, "????.??.01")]
    #[test_case(Date { year: oru16!(), month: oru8!(), day: oru8!() }, "????.??.??")]
    #[test_case(Date { year: oru16!(1), month: oru8!(), day: oru8!() }, "0001.??.??")]
    #[test_case(Date { year: oru16!(), month: oru8!(12), day: oru8!() }, "????.12.??")]
    fn to_string_from_string(date: Date, date_str: &str) {
        assert_eq!(date.to_string(), date_str);
        assert_eq!(Date::from_str(date_str).unwrap(), date);
    }

    #[cfg(feature = "time")]
    #[test_case(Ok(::time::Date::from_calendar_date(2000, ::time::Month::October, 15).unwrap()), Date { year: oru16!(2000), month: oru8!(10), day: oru8!(15) })]
    fn time(correct: Result<::time::Date, IntoTimeDateError>, test: Date) {
        assert_eq!(correct, ::time::Date::try_from(test));
    }
}